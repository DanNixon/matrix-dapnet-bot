use super::BotCommand;
use crate::Config;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;
use matrix_sdk::ruma::{events::room::message::TextMessageEventContent, UserId};

#[derive(Debug, Parser)]
pub(super) struct TransmitterGroup {}

#[async_trait]
impl BotCommand for TransmitterGroup {
    async fn run_command(
        &self,
        _: UserId,
        dapnet: dapnet_api::Client,
        _: Config,
    ) -> Result<TextMessageEventContent> {
        match dapnet.get_all_transmitter_groups().await? {
            Some(groups) => Ok(TextMessageEventContent::markdown(format!(
                "**Transmitter Groups**: {}",
                groups
                    .into_iter()
                    .map(|g| g.name)
                    .collect::<Vec<String>>()
                    .join(", "),
            ))),
            None => Err(anyhow! {"Failed to query transmitter groups"}),
        }
    }
}
