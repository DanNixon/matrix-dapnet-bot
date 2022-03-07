use super::BotCommand;
use crate::Config;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use matrix_sdk::ruma::{events::room::message::TextMessageEventContent, UserId};

#[derive(Debug, Parser)]
pub(super) struct Node {
    /// Name of the node to lookup
    name: String,
}

#[async_trait]
impl BotCommand for Node {
    async fn run_command(
        &self,
        _: UserId,
        dapnet: dapnet_api::Client,
        _: Config,
    ) -> Result<TextMessageEventContent> {
        match dapnet.get_node(&self.name).await? {
            Some(node) => Ok(TextMessageEventContent::markdown(format!(
                "**Node** {}<br>\
                Status: {:?}<br>\
                Version: {}<br>\
                Owner(s): {}<br>\
                Position: {}, {}<br>\
                Connection: {}<br>",
                node.name,
                node.status,
                node.version,
                node.owners.join(", "),
                node.latitude,
                node.longitude,
                match node.connection {
                    Some(c) => format!("{}", c),
                    None => "none".to_string(),
                }
            ))),
            None => Ok(TextMessageEventContent::plain(format!(
                "Node \"{}\" not found.",
                &self.name,
            ))),
        }
    }
}
