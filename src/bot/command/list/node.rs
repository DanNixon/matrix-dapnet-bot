use super::BotCommand;
use crate::Config;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;
use matrix_sdk::ruma::{events::room::message::TextMessageEventContent, UserId};

#[derive(Debug, Parser)]
pub(super) struct Node {}

#[async_trait]
impl BotCommand for Node {
    async fn run_command(
        &self,
        _: UserId,
        dapnet: dapnet_api::Client,
        _: Config,
    ) -> Result<TextMessageEventContent> {
        match dapnet.get_all_nodes().await? {
            Some(nodes) => Ok(TextMessageEventContent::markdown(format!(
                "**Nodes**: {}",
                nodes
                    .into_iter()
                    .map(|n| n.name)
                    .collect::<Vec<String>>()
                    .join(", "),
            ))),
            None => Err(anyhow!("Failed to query nodes")),
        }
    }
}
