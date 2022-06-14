mod node;
mod rubric;
mod transmitter_group;

use super::BotCommand;
use crate::Config;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use matrix_sdk::ruma::{events::room::message::RoomMessageEventContent, OwnedUserId};

#[derive(Debug, Parser)]
pub(crate) struct List {
    #[clap(subcommand)]
    command: Subcommand,
}

#[async_trait]
impl BotCommand for List {
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
    Node(node::Node),
    Rubric(rubric::Rubric),
    TransmitterGroup(transmitter_group::TransmitterGroup),
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
            Subcommand::Node(c) => c.run_command(sender, dapnet, config).await,
            Subcommand::Rubric(c) => c.run_command(sender, dapnet, config).await,
            Subcommand::TransmitterGroup(c) => c.run_command(sender, dapnet, config).await,
        }
    }
}
