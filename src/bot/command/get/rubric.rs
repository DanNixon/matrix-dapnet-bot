use super::BotCommand;
use crate::Config;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use matrix_sdk::ruma::{events::room::message::TextMessageEventContent, UserId};

#[derive(Debug, Parser)]
pub(super) struct Rubric {
    /// Name of the rubric to lookup
    name: String,
}

#[async_trait]
impl BotCommand for Rubric {
    async fn run_command(
        &self,
        _: UserId,
        dapnet: dapnet_api::Client,
        _: Config,
    ) -> Result<TextMessageEventContent> {
        match dapnet.get_rubric(&self.name).await? {
            Some(rubric) => Ok(TextMessageEventContent::markdown(format!(
                "**Rubric** {}<br>\
                RIC: {}<br>\
                Label: {}<br>\
                Owner(s): {}<br>\
                Transmitter group(s): {}",
                rubric.name,
                rubric.number,
                rubric.label,
                rubric.owners.join(", "),
                rubric.transmitter_groups.join(", "),
            ))),
            None => Ok(TextMessageEventContent::plain(format!(
                "Rubric \"{}\" not found.",
                &self.name,
            ))),
        }
    }
}
