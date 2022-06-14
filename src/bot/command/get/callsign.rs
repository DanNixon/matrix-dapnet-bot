use super::BotCommand;
use crate::Config;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use matrix_sdk::ruma::{events::room::message::RoomMessageEventContent, OwnedUserId};

#[derive(Debug, Parser)]
pub(super) struct Callsign {
    /// The callsign to lookup
    #[clap(value_parser)]
    callsign: String,
}

#[async_trait]
impl BotCommand for Callsign {
    async fn run_command(
        &self,
        _: OwnedUserId,
        dapnet: dapnet_api::Client,
        _: Config,
    ) -> Result<RoomMessageEventContent> {
        match dapnet.get_callsign(&self.callsign).await? {
            Some(callsign) => Ok(RoomMessageEventContent::text_markdown(format!(
                "**Callsign** {}<br>\
                {}",
                callsign.name, callsign.description,
            ))),
            None => Ok(RoomMessageEventContent::text_plain(format!(
                "Callsign \"{}\" not found.",
                &self.callsign,
            ))),
        }
    }
}
