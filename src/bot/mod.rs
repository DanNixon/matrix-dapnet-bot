mod command;

use crate::metrics::{CommandKind, CommandLables, COMMANDS};
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use matrix_sdk::ruma::{events::room::message::RoomMessageEventContent, OwnedUserId};

#[async_trait]
pub(crate) trait BotCommand {
    async fn run_command(
        &self,
        sender: OwnedUserId,
        dapnet: dapnet_api::Client,
    ) -> Result<RoomMessageEventContent>;
}

/// Hello, I'm a helpful bot that lets you interact with DAPNET from the comfort of Matrix.
/// For more information about me, please see https://github.com/DanNixon/matrix-dapnet-bot.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Bot {
    #[clap(subcommand)]
    command: Subcommand,
}

#[async_trait]
impl BotCommand for Bot {
    async fn run_command(
        &self,
        sender: OwnedUserId,
        dapnet: dapnet_api::Client,
    ) -> Result<RoomMessageEventContent> {
        self.command.run_command(sender, dapnet).await
    }
}

#[derive(Debug, Parser)]
enum Subcommand {
    /// Get the details of the operator responsible for this instance of the bot
    BotOperators(command::BotOperators),

    /// Checks if your Matrix ID is registered to transmit calls over RF
    TxCheck(command::TxCheck),

    /// List resources by type
    List(command::List),

    /// Get details of a specific resource
    Get(command::Get),

    /// Get node, user, transmitter, rubric and call metrics
    Stats(command::Stats),

    /// Send calls/messages
    Send(command::Send),

    /// Send rubric news/content
    SendNews(command::SendNews),
}

#[async_trait]
impl BotCommand for Subcommand {
    async fn run_command(
        &self,
        sender: OwnedUserId,
        dapnet: dapnet_api::Client,
    ) -> Result<RoomMessageEventContent> {
        COMMANDS
            .get_or_create(&CommandLables::new(
                &sender,
                match self {
                    Subcommand::List(_) => CommandKind::List,
                    Subcommand::Get(_) => CommandKind::Get,
                    Subcommand::Stats(_) => CommandKind::Stats,
                    Subcommand::Send(_) => CommandKind::Send,
                    Subcommand::SendNews(_) => CommandKind::SendNews,
                },
            ))
            .inc();

        match self {
            Subcommand::List(c) => c.run_command(sender, dapnet).await,
            Subcommand::Get(c) => c.run_command(sender, dapnet).await,
            Subcommand::Stats(c) => c.run_command(sender, dapnet).await,
            Subcommand::Send(c) => c.run_command(sender, dapnet).await,
            Subcommand::SendNews(c) => c.run_command(sender, dapnet).await,
        }
    }
}
