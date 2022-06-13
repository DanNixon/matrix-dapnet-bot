use super::BotCommand;
use crate::Config;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use matrix_sdk::ruma::{events::room::message::TextMessageEventContent, UserId};

#[derive(Debug, Parser)]
pub(super) struct Transmitter {
    /// Name of the transmitter to lookup
    #[clap(value_parser)]
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
            Some(transmitter) => Ok(TextMessageEventContent::markdown(format!(
                "**Transmitter** {}<br>\
                Owner(s): {}<br>\
                Usage: {:?}<br>\
                Timeslots: {}<br>\
                Status:<br>\
                - {:?}<br>\
                - Calls: {}<br>\
                - Last update: {:?}<br>\
                - Last connected: {:?}<br>\
                - Connected since: {:?}<br>\
                Device:<br>\
                - Type: {}<br>\
                - Version: {}<br>\
                - Tx power: {}W<br>\
                Antenna:<br>\
                - Height above ground: {}m<br>\
                - Type: {:?}<br>\
                - Direction: {}<br>\
                - Gain: {}dBi<br>\
                [Location](https://www.openstreetmap.org/#map=18/{}/{})",
                transmitter.name,
                transmitter.owners.join(", "),
                transmitter.usage,
                transmitter.timeslots,
                transmitter.status,
                transmitter.call_count,
                transmitter.last_update,
                transmitter.last_connected,
                transmitter.connected_since,
                transmitter
                    .device_type
                    .unwrap_or_else(|| "(unknown)".to_string()),
                transmitter
                    .device_version
                    .unwrap_or_else(|| "(unknown)".to_string()),
                transmitter.power,
                transmitter.antenna_height_above_ground,
                transmitter.antenna_type,
                transmitter.antenna_direction,
                transmitter.antenna_gain,
                transmitter.latitude,
                transmitter.longitude,
            ))),
            None => Ok(TextMessageEventContent::plain(format!(
                "Transmitter \"{}\" not found.",
                &self.name,
            ))),
        }
    }
}
