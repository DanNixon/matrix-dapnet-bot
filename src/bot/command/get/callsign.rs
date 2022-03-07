use super::BotCommand;
use crate::Config;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use matrix_sdk::ruma::{events::room::message::TextMessageEventContent, UserId};

#[derive(Debug, Parser)]
pub(super) struct Callsign {
    /// The callsign to lookup
    callsign: String,
}

#[async_trait]
impl BotCommand for Callsign {
    async fn run_command(
        &self,
        _: UserId,
        dapnet: dapnet_api::Client,
        _: Config,
    ) -> Result<TextMessageEventContent> {
        match dapnet.get_callsign(&self.callsign).await? {
            Some(callsign) => Ok(TextMessageEventContent::markdown(format!(
                "**Callsign** {}<br>\
                {}",
                callsign.name, callsign.description,
            ))),
            None => Ok(TextMessageEventContent::plain(format!(
                "Callsign \"{}\" not found.",
                &self.callsign,
            ))),
        }
    }
}
