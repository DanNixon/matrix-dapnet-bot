use super::BotCommand;
use crate::Config;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;
use matrix_sdk::ruma::{events::room::message::RoomMessageEventContent, OwnedUserId};

#[derive(Debug, Parser)]
pub(super) struct Rubric {}

#[async_trait]
impl BotCommand for Rubric {
    async fn run_command(
        &self,
        _: OwnedUserId,
        dapnet: dapnet_api::Client,
        _: Config,
    ) -> Result<RoomMessageEventContent> {
        match dapnet.get_all_rubrics().await? {
            Some(nodes) => Ok(RoomMessageEventContent::text_markdown(format!(
                "**Rubrics**: {}",
                nodes
                    .into_iter()
                    .map(|n| n.name)
                    .collect::<Vec<String>>()
                    .join(", "),
            ))),
            None => Err(anyhow!("Failed to query rubrics")),
        }
    }
}
