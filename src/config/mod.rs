mod callsign;
mod user;

use anyhow::Result;
pub(crate) use callsign::Callsign;
use matrix_sdk::ruma::UserId;
use serde::Deserialize;
use std::fs;
use user::User;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Config {
    pub operators: Vec<User>,
    pub users: Vec<User>,
}

impl Config {
    pub fn from_file(filename: &str) -> Result<Self> {
        Ok(toml::from_str(&fs::read_to_string(filename)?)?)
    }

    pub fn get_user(&self, matrix_id: &UserId) -> Option<&User> {
        self.users
            .iter()
            .find(|u| u.matrix_ids.iter().any(|i| i == matrix_id))
    }

    pub fn check_user_can_transmit(
        &self,
        matrix_id: &UserId,
        callsign: &Callsign,
    ) -> Option<&User> {
        self.users
            .iter()
            .find(|u| u.check_can_transmit(matrix_id, callsign).is_ok())
    }
}
