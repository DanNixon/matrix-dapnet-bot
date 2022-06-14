use super::BotCommand;
use crate::Config;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use itertools::Itertools;
use matrix_sdk::ruma::{events::room::message::RoomMessageEventContent, OwnedUserId};

#[derive(Debug, Parser)]
pub(super) struct Rubric {
    /// Name of the rubric to lookup
    #[clap(value_parser)]
    name: String,
}

#[async_trait]
impl BotCommand for Rubric {
    async fn run_command(
        &self,
        _: OwnedUserId,
        dapnet: dapnet_api::Client,
        _: Config,
    ) -> Result<RoomMessageEventContent> {
        match dapnet.get_rubric(&self.name).await? {
            Some(rubric) => {
                let news = match dapnet.get_news(&rubric.name).await {
                    Ok(news) => match news {
                        Some(news) => news
                            .iter()
                            .map(|i| {
                                format!(
                                    "-{} {}{}{}",
                                    match i.number {
                                        Some(t) => format!(" ({})", t),
                                        None => String::default(),
                                    },
                                    i.text,
                                    match i.timestamp {
                                        Some(t) => format!(" @ {}", t),
                                        None => String::default(),
                                    },
                                    match &i.sender {
                                        Some(t) => format!(" by {}", t),
                                        None => String::default(),
                                    }
                                )
                            })
                            .join("\n"),
                        None => {
                            log::error!("Failed to fetch news for rubric {}", rubric.name);
                            "(failed to query news)".to_string()
                        }
                    },
                    Err(e) => {
                        log::error!("Failed to fetch news for rubric {}: {}", rubric.name, e);
                        "(failed to query news)".to_string()
                    }
                };

                Ok(RoomMessageEventContent::text_markdown(format!(
                    "**Rubric** {}<br>\
                    RIC: {}<br>\
                    Label: {}<br>\
                    Owner(s): {}<br>\
                    Transmitter group(s): {}<br>\
                    News:\n\
                    {}",
                    rubric.name,
                    rubric.number,
                    rubric.label,
                    rubric.owners.join(", "),
                    rubric.transmitter_groups.join(", "),
                    news,
                )))
            }
            None => Ok(RoomMessageEventContent::text_plain(format!(
                "Rubric \"{}\" not found.",
                &self.name,
            ))),
        }
    }
}
