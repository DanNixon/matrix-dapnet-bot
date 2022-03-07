use super::BotCommand;
use crate::Config;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use matrix_sdk::ruma::{events::room::message::TextMessageEventContent, UserId};

#[derive(Debug, Parser)]
pub(super) struct Transmitter {
    /// Name of the transmitter to lookup
    name: String,
}

#[async_trait]
impl BotCommand for Transmitter {
    async fn run_command(
        &self,
        _: UserId,
        dapnet: dapnet_api::Client,
        _: Config,
    ) -> Result<TextMessageEventContent> {
        match dapnet.get_transmitter(&self.name).await? {
            // TODO: all info
            Some(transmitter) => Ok(TextMessageEventContent::markdown(format!(
                "**Transmitter** {}<br>\
                Status: {:?}<br>\
                Timeslots: {}<br>\
                Device: {:?} (version {:?})<br>\
                Usage: {:?}<br>\
                Owner(s): {}<br>\
                (this response is a WIP)
                ",
                transmitter.name,
                transmitter.status,
                transmitter.timeslots,
                transmitter.device_type,
                transmitter.device_version,
                transmitter.usage,
                transmitter.owners.join(", "),
            ))),
            None => Ok(TextMessageEventContent::plain(format!(
                "Transmitter \"{}\" not found.",
                &self.name,
            ))),
        }
    }
}
