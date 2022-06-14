use super::{utils, BotCommand};
use crate::{Callsign, Config};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;
use dapnet_api::{News, Rubric};
use matrix_sdk::ruma::{events::room::message::RoomMessageEventContent, OwnedUserId};
use std::collections::HashSet;

#[derive(Debug, Parser)]
pub(crate) struct SendNews {
    /// Transmitting callsign (if not provided, the first callsign configured for your Matrix ID
    /// will be used)
    #[clap(value_parser, long, short)]
    from: Option<Callsign>,

    /// News item number (1-10) (mainly for Skyper pagers)
    #[clap(value_parser, long, short, default_value = "1")]
    number: i8,

    /// Name of the rubric to publish news to
    #[clap(value_parser)]
    rubric: String,

    #[clap(value_parser)]
    message: Vec<String>,
}

fn check_user_can_send_to_rubric(
    sender: &OwnedUserId,
    config: &Config,
    rubric: &Rubric,
) -> Result<()> {
    let user = config.get_user(sender).unwrap();

    let user_usernames = user.dapnet_usernames.iter().collect::<HashSet<_>>();
    let allowed_usernames = rubric.owners.iter().collect::<HashSet<_>>();

    if user_usernames.intersection(&allowed_usernames).count() > 0 {
        Ok(())
    } else {
        Err(anyhow!(
            "User {} is not permitted to send to rubric {}",
            sender,
            rubric.name
        ))
    }
}

#[async_trait]
impl BotCommand for SendNews {
    async fn run_command(
        &self,
        sender: OwnedUserId,
        dapnet: dapnet_api::Client,
        config: Config,
    ) -> Result<RoomMessageEventContent> {
        let message = &self.message.join(" ");
        let transmit_callsign = utils::get_transmit_callsign(&sender, &config, &self.from)?;

        let rubric = match dapnet.get_rubric(&self.rubric).await? {
            Some(r) => r,
            None => {
                return Err(anyhow!(
                    "Could not find rubric with name \"{}\"",
                    self.rubric
                ));
            }
        };

        check_user_can_send_to_rubric(&sender, &config, &rubric)?;

        log::info!(
            "Request to send news: \"{}\" to rubric {} from {}, with options: {:?}",
            message,
            rubric.name,
            transmit_callsign,
            &self
        );

        let mut news = News::new(
            rubric.name.clone(),
            format!("{}: {}", transmit_callsign, message),
        );
        news.number = Some(self.number);

        match dapnet.new_news(&news).await {
            Ok(()) => Ok(RoomMessageEventContent::text_markdown(format!(
                "{}, your news item has been sent to rubric {} ({})!",
                sender, rubric.name, rubric.number,
            ))),
            Err(e) => Err(e),
        }
    }
}
