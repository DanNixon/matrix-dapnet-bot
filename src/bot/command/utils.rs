use anyhow::{anyhow, Result};
use matrix_sdk::ruma::UserId;

pub(crate) fn get_transmit_callsign<'a>(
    sender: &'a UserId,
    requested_callsign: &'a Option<Callsign>,
) -> Result<&'a Callsign> {
    match requested_callsign {
        Some(callsign) => match config.check_user_can_transmit(sender, callsign) {
            Some(_) => Ok(callsign),
            None => Err(anyhow!(
                "{} is not permitted to use callsign {}",
                sender,
                callsign
            )),
        },
        None => match config.get_user(sender) {
            Some(user) => match user.get_default_callsign() {
                Some(callsign) => Ok(callsign),
                None => Err(anyhow!("{} has no configured callsigns", sender)),
            },
            None => Err(anyhow!("{} is not a configured user", sender)),
        },
    }
}
