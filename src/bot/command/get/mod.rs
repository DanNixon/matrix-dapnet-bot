mod callsign;
mod node;
mod rubric;
mod transmitter;
mod transmitter_group;

use super::BotCommand;
use crate::Config;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use matrix_sdk::ruma::{events::room::message::TextMessageEventContent, UserId};

#[derive(Debug, Parser)]
pub(crate) struct Get {
    #[clap(subcommand)]
    command: Subcommand,
}

#[async_trait]
impl BotCommand for Get {
    async fn run_command(
        &self,
        sender: UserId,
        dapnet: dapnet_api::Client,
        config: Config,
    ) -> Result<TextMessageEventContent> {
        self.command.run_command(sender, dapnet, config).await
    }
}

#[derive(Debug, Parser)]
enum Subcommand {
    Callsign(callsign::Callsign),
    Node(node::Node),
    Rubric(rubric::Rubric),
    Transmitter(transmitter::Transmitter),
    TransmitterGroup(transmitter_group::TransmitterGroup),
}

#[async_trait]
impl BotCommand for Subcommand {
    async fn run_command(
        &self,
        sender: UserId,
        dapnet: dapnet_api::Client,
        config: Config,
    ) -> Result<TextMessageEventContent> {
        match self {
            Subcommand::Callsign(c) => c.run_command(sender, dapnet, config).await,
            Subcommand::Node(c) => c.run_command(sender, dapnet, config).await,
            Subcommand::Rubric(c) => c.run_command(sender, dapnet, config).await,
            Subcommand::Transmitter(c) => c.run_command(sender, dapnet, config).await,
            Subcommand::TransmitterGroup(c) => c.run_command(sender, dapnet, config).await,
        }
    }
}
