use super::BotCommand;
use crate::Config;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use matrix_sdk::ruma::{events::room::message::TextMessageEventContent, UserId};

#[derive(Debug, Parser)]
pub(super) struct TransmitterGroup {
    /// Name of the transmitter group to lookup
    name: String,
}

#[async_trait]
impl BotCommand for TransmitterGroup {
    async fn run_command(
        &self,
        _: UserId,
        dapnet: dapnet_api::Client,
        _: Config,
    ) -> Result<TextMessageEventContent> {
        match dapnet.get_transmitter_group(&self.name).await? {
            Some(group) => Ok(TextMessageEventContent::markdown(format!(
                "**Transmitter Group**: {}<br>\
                Description: {}<br>\
                Owner(s): {}<br>\
                Transmitter(s): {}<br>",
                group.name,
                group.description,
                group.owners.join(", "),
                group.transmitters.join(", "),
            ))),
            None => Ok(TextMessageEventContent::plain(format!(
                "Transmitter Group \"{}\" not found.",
                &self.name,
            ))),
        }
    }
}
