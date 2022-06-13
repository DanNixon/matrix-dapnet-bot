use super::BotCommand;
use crate::{Callsign, Config};
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use itertools::Itertools;
use matrix_sdk::ruma::{events::room::message::TextMessageEventContent, UserId};

#[derive(Debug, Parser)]
pub(crate) struct TxCheck {
    /// Transmitting callsign (if not provided, the first callsign configured for your Matrix ID
    /// will be used)
    #[clap(value_parser)]
    callsign: Option<Callsign>,
}

#[async_trait]
impl BotCommand for TxCheck {
    async fn run_command(
        &self,
        sender: UserId,
        _: dapnet_api::Client,
        config: Config,
    ) -> Result<TextMessageEventContent> {
        Ok(TextMessageEventContent::markdown(match &self.callsign {
            Some(callsign) => match config.check_user_can_transmit(&sender, callsign) {
                Some(user) => format!(
                    "Congrats {}, you are configured to transmit using the following callsigns: {}",
                    sender,
                    user.callsigns.iter().format(", ")
                ),
                None => format!(
                    "Sorry {}, you are not configured to transmit with callsign {}.",
                    sender, callsign
                ),
            },
            None => match config.get_user(&sender) {
                Some(user) => format!(
                    "Congrats {}, you are configured to transmit using the following callsigns: {}",
                    sender, user.callsigns.iter().format(", ")
                ),
                None => format!(
                    "Sorry {}, I'm afraid I do not know you, if you hold an ameatur radio license speak to an operator of this bot to be allowed to transmit.",
                    sender
                ),
            }
        }))
    }
}
