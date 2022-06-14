mod command;

use crate::Config;
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
        config: Config,
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
        config: Config,
    ) -> Result<RoomMessageEventContent> {
        self.command.run_command(sender, dapnet, config).await
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
        config: Config,
    ) -> Result<RoomMessageEventContent> {
        match self {
            Subcommand::BotOperators(c) => c.run_command(sender, dapnet, config).await,
            Subcommand::TxCheck(c) => c.run_command(sender, dapnet, config).await,
            Subcommand::List(c) => c.run_command(sender, dapnet, config).await,
            Subcommand::Get(c) => c.run_command(sender, dapnet, config).await,
            Subcommand::Stats(c) => c.run_command(sender, dapnet, config).await,
            Subcommand::Send(c) => c.run_command(sender, dapnet, config).await,
            Subcommand::SendNews(c) => c.run_command(sender, dapnet, config).await,
        }
    }
}
