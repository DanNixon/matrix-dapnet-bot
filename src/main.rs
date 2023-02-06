mod bot;
mod config;
mod metrics;

use anyhow::Result;
use bot::{Bot, BotCommand};
use clap::Parser;
use config::{Callsign, Config};
use kagiyama::Watcher;
use matrix_sdk::{
    self,
    config::SyncSettings,
    room::Room,
    ruma::{
        events::room::message::{
            MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent,
            TextMessageEventContent,
        },
        OwnedUserId, UserId,
    },
};
use std::net::SocketAddr;

/// A Matrix bot allowing messages to be sent via DAPNET
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Matrix username
    #[clap(value_parser, long, env = "MATRIX_USERNAME")]
    matrix_username: String,

    /// Matrix password
    #[clap(value_parser, long, env = "MATRIX_PASSWORD")]
    matrix_password: String,

    /// DAPNET username
    #[clap(value_parser, long, env = "DAPNET_USERNAME")]
    dapnet_username: String,

    /// DAPNET password
    #[clap(value_parser, long, env = "DAPNET_PASSWORD")]
    dapnet_password: String,

    /// Path to configuration file
    #[clap(
        value_parser,
        long,
        env = "CONFIG_FILE",
        default_value = "./config.toml"
    )]
    config_file: String,

    /// Address to listen on for observability/metrics endpoints
    #[clap(
        value_parser,
        long,
        env = "OBSERVABILITY_ADDRESS",
        default_value = "127.0.0.1:9090"
    )]
    observability_address: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Cli::parse();
    let config = Config::from_file(&args.config_file)?;

    let mut watcher = Watcher::<metrics::ReadinessConditions>::default();
    metrics::register(&watcher);
    watcher.start_server(args.observability_address).await?;

    let dapnet_client = dapnet_api::Client::new(&args.dapnet_username, &args.dapnet_password);

    log::info!("Logging into Matrix...");
    let matrix_user = UserId::parse(args.matrix_username.clone())?;
    let matrix_client = matrix_sdk::Client::builder()
        .homeserver_url(format!("https://{}", matrix_user.server_name()))
        .build()
        .await?;
    matrix_client
        .login(
            matrix_user.localpart(),
            &args.matrix_password,
            None,
            Some("matrix-dapnet-bot"),
        )
        .await?;

    log::info!("Performing initial sync...");
    matrix_client.sync_once(SyncSettings::default()).await?;

    matrix_client
        .register_event_handler({
            let dapnet_client = dapnet_client.clone();
            let config = config.clone();
            let matrix_user = matrix_user.clone();
            move |event: OriginalSyncRoomMessageEvent, room: Room| {
                let dapnet_client = dapnet_client.clone();
                let config = config.clone();
                let matrix_user = matrix_user.clone();
                async move { handle_message(event, room, matrix_user, dapnet_client, config).await }
            }
        })
        .await;

    log::info!("Logged into Matrix");
    watcher
        .readiness_probe()
        .mark_ready(metrics::ReadinessConditions::LoggedIntoMatrix);

    matrix_client
        .sync(SyncSettings::default().token(matrix_client.sync_token().await.unwrap()))
        .await;

    Ok(())
}

async fn handle_message(
    event: OriginalSyncRoomMessageEvent,
    room: Room,
    me: OwnedUserId,
    dapnet: dapnet_api::Client,
    config: Config,
) {
    if let Room::Joined(room) = room {
        if let MessageType::Text(TextMessageEventContent { body, .. }) = event.content.msgtype {
            if event.sender != me && body.starts_with("!dapnet") {
                if let Err(e) = room
                    .send(
                        match Bot::try_parse_from(body.split(' ')) {
                            Ok(args) => {
                                match args.run_command(event.sender, dapnet, config).await {
                                    Ok(reply) => reply,
                                    Err(e) => {
                                        metrics::FAILURES.inc();
                                        RoomMessageEventContent::text_markdown(format!(
                                            "**Sad bot is sad :c**\n```\n{e}\n```",
                                        ))
                                    }
                                }
                            }
                            Err(e) => {
                                metrics::FAILURES.inc();
                                RoomMessageEventContent::text_markdown(format!("```\n{e}\n```"))
                            }
                        },
                        None,
                    )
                    .await
                {
                    metrics::FAILURES.inc();
                    room.send(
                        RoomMessageEventContent::text_markdown(format!(
                            "**Sad bot is sad :c**\n```\n{e}\n```",
                        )),
                        None,
                    )
                    .await
                    .unwrap();
                }
            }
        }
    }
}
