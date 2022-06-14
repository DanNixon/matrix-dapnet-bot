mod bot;
mod config;

use anyhow::Result;
use bot::{Bot, BotCommand};
use clap::Parser;
use config::{Callsign, Config};
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
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Cli::parse();

    let dapnet_client = dapnet_api::Client::new(&args.dapnet_username, &args.dapnet_password);

    let config = Config::from_file(&args.config_file)?;
    log::debug!("Loaded configuration: {:?}", config);

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
                                    Err(e) => RoomMessageEventContent::text_markdown(format!(
                                        "**Sad bot is sad :c**\n```\n{}\n```",
                                        e
                                    )),
                                }
                            }
                            Err(e) => {
                                RoomMessageEventContent::text_markdown(format!("```\n{}\n```", e))
                            }
                        },
                        None,
                    )
                    .await
                {
                    room.send(
                        RoomMessageEventContent::text_markdown(format!(
                            "**Sad bot is sad :c**\n```\n{}\n```",
                            e
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
