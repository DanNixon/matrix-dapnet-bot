use super::{utils, BotCommand};
use crate::{Callsign, Config};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;
use dapnet_api::Call;
use matrix_sdk::ruma::{events::room::message::TextMessageEventContent, UserId};

#[derive(Debug, Parser)]
pub(crate) struct Send {
    /// Transmitting callsign (if not provided, the first callsign configured for your Matrix ID
    /// will be used)
    #[clap(long, short)]
    from: Option<Callsign>,

    /// Destination callsign(s)
    #[clap(long, short)]
    recipient: Vec<Callsign>,

    /// Names of transmitter groups to send via
    #[clap(long, short, default_value = "all")]
    via: Vec<String>,

    /// Should message be sent with high priority
    #[clap(long)]
    emergency: bool,

    message: Vec<String>,
}

#[async_trait]
impl BotCommand for Send {
    async fn run_command(
        &self,
        sender: UserId,
        dapnet: dapnet_api::Client,
        config: Config,
    ) -> Result<TextMessageEventContent> {
        let message = &self.message.join(" ");
        let transmit_callsign = utils::get_transmit_callsign(&sender, &config, &self.from)?;

        if self.recipient.is_empty() {
            return Err(anyhow!("At least one recipient must be specified"));
        }

        log::info!(
            "Request to send message: \"{}\" from {}, with options: {:?}",
            message,
            transmit_callsign,
            &self
        );

        match dapnet
            .new_call(&Call::new(
                format!("{}: {}", transmit_callsign, message),
                self.recipient
                    .iter()
                    .map(|r| r.lower().to_string())
                    .collect(),
                self.via.clone(),
            ))
            .await
        {
            Ok(()) => Ok(TextMessageEventContent::markdown(format!(
                "{}, your message has been sent!",
                sender
            ))),
            Err(e) => Err(e),
        }
    }
}
