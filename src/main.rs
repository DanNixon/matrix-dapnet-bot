mod bot;
mod config;

use anyhow::Result;
use bot::{Bot, BotCommand};
use clap::Parser;
use config::{Callsign, Config};
use matrix_sdk::{
    self,
    room::Room,
    ruma::{
        events::{
            room::message::{MessageEventContent, MessageType, TextMessageEventContent},
            AnyMessageEventContent, SyncMessageEvent,
        },
        UserId,
    },
    SyncSettings,
};

fn get_message_body(event: &SyncMessageEvent<MessageEventContent>) -> Option<&String> {
    if let SyncMessageEvent {
        content:
            MessageEventContent {
                msgtype: MessageType::Text(TextMessageEventContent { body, .. }),
                ..
            },
        ..
    } = event
    {
        Some(body)
    } else {
        None
    }
}

async fn handle_message(
    event: SyncMessageEvent<MessageEventContent>,
    room: Room,
    me: UserId,
    dapnet: dapnet_api::Client,
    config: Config,
) {
    if let Room::Joined(room) = room {
        if let Some(msg_body) = get_message_body(&event) {
            if event.sender != me && msg_body.starts_with("!dapnet") {
                if let Err(e) = room
                    .send(
                        AnyMessageEventContent::RoomMessage(MessageEventContent::new(
                            MessageType::Text(match Bot::try_parse_from(msg_body.split(' ')) {
                                Ok(args) => {
                                    match args.run_command(event.sender, dapnet, config).await {
                                        Ok(reply) => reply,
                                        Err(e) => TextMessageEventContent::markdown(format!(
                                            "**Sad bot is sad :c**\n```\n{}\n```",
                                            e
                                        )),
                                    }
                                }
                                Err(e) => {
                                    TextMessageEventContent::markdown(format!("```\n{}\n```", e))
                                }
                            }),
                        )),
                        None,
                    )
                    .await
                {
                    room.send(
                        AnyMessageEventContent::RoomMessage(MessageEventContent::new(
                            MessageType::Text(TextMessageEventContent::markdown(format!(
                                "**Sad bot is sad :c**\n```\n{}\n```",
                                e
                            ))),
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

/// A Matrix bot allowing messages to be sent via DAPNET
#[derive(Debug, Parser)]
struct Cli {
    /// Matrix username
    #[clap(long, env = "MATRIX_USERNAME")]
    matrix_username: String,

    /// Matrix password
    #[clap(long, env = "MATRIX_PASSWORD")]
    matrix_password: String,

    /// DAPNET username
    #[clap(long, env = "DAPNET_USERNAME")]
    dapnet_username: String,

    /// DAPNET password
    #[clap(long, env = "DAPNET_PASSWORD")]
    dapnet_password: String,

    /// Path to configuration file
    #[clap(long, env = "CONFIG_FILE", default_value = "./config.toml")]
    config_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Cli::parse();

    let dapnet_client = dapnet_api::Client::new(&args.dapnet_username, &args.dapnet_password);

    let config = Config::from_file(&args.config_file)?;
    log::debug! {"Loaded configuration: {:?}", config};

    log::info! {"Logging into Matrix..."};
    let matrix_user = UserId::try_from(args.matrix_username.clone())?;
    let matrix_client = matrix_sdk::Client::new_from_user_id(matrix_user.clone()).await?;
    matrix_client
        .login(
            matrix_user.localpart(),
            &args.matrix_password,
            None,
            Some("matrix-dapnet-bot"),
        )
        .await?;

    log::info! {"Performing initial sync..."};
    matrix_client.sync_once(SyncSettings::default()).await?;

    matrix_client
        .register_event_handler({
            let dapnet_client = dapnet_client.clone();
            let config = config.clone();
            let matrix_user = matrix_user.clone();
            move |event: SyncMessageEvent<MessageEventContent>, room: Room| {
                let dapnet_client = dapnet_client.clone();
                let config = config.clone();
                let matrix_user = matrix_user.clone();
                async move {
                    handle_message(event, room, matrix_user, dapnet_client, config).await;
                }
            }
        })
        .await;

    log::info! {"Logged into Matrix"};
    matrix_client
        .sync(SyncSettings::default().token(matrix_client.sync_token().await.unwrap()))
        .await;

    Ok(())
}
