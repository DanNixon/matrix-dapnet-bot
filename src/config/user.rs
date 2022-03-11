use super::Callsign;
use anyhow::{anyhow, Result};
use matrix_sdk::ruma::UserId;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct User {
    pub matrix_ids: Vec<UserId>,
    pub callsigns: Vec<Callsign>,
    #[serde(default)]
    pub dapnet_usernames: Vec<String>,
}

impl User {
    pub(crate) fn check_can_transmit(&self, matrix_id: &UserId, callsign: &Callsign) -> Result<()> {
        if !self.callsigns.contains(callsign) {
            Err(anyhow! {"Callsign \"{}\" unknown to this user", callsign})
        } else if !self.matrix_ids.contains(matrix_id) {
            Err(anyhow! {"Matrix user ID \"{}\" unknown to this user", matrix_id})
        } else {
            Ok(())
        }
    }

    pub(crate) fn get_default_callsign(&self) -> Option<&Callsign> {
        self.callsigns.get(0)
    }
}
