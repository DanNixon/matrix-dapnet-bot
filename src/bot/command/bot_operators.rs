use super::BotCommand;
use crate::Config;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use itertools::Itertools;
use matrix_sdk::ruma::{events::room::message::TextMessageEventContent, UserId};

#[derive(Debug, Parser)]
pub(crate) struct BotOperators {}

#[async_trait]
impl BotCommand for BotOperators {
    async fn run_command(
        &self,
        _: UserId,
        _: dapnet_api::Client,
        config: Config,
    ) -> Result<TextMessageEventContent> {
        Ok(TextMessageEventContent::markdown(format!(
            "This bot is managed by:\n{}",
            config
                .operators
                .into_iter()
                .map(|op| format!(
                    "- {} ({})",
                    op.matrix_ids
                        .into_iter()
                        .map(|id| id.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                    op.callsigns.iter().format(",")
                ))
                .collect::<Vec<String>>()
                .join("\n")
        )))
    }
}
