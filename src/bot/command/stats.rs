use super::BotCommand;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;
use matrix_sdk::ruma::{events::room::message::RoomMessageEventContent, OwnedUserId};

#[derive(Debug, Parser)]
pub(crate) struct Stats {}

#[async_trait]
impl BotCommand for Stats {
    async fn run_command(
        &self,
        _: OwnedUserId,
        dapnet: dapnet_api::Client,
    ) -> Result<RoomMessageEventContent> {
        match dapnet.get_statistics().await? {
            Some(stats) => Ok(RoomMessageEventContent::text_markdown(format!(
                "**Statistics**<br>\
                Calls: {}<br>\
                Callsigns: {}<br>\
                News: {}<br>\
                Rubrics: {}<br>\
                Transmitters online: {}/{}<br>\
                Nodes online: {}/{}<br>\
                Users: {}",
                stats.calls,
                stats.callsigns,
                stats.news,
                stats.rubrics,
                stats.transmitters_online,
                stats.transmitters_total,
                stats.nodes_online,
                stats.nodes_total,
                stats.users,
            ))),
            None => Err(anyhow!("Failed to query statistics")),
        }
    }
}
