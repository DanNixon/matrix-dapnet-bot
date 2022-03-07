use anyhow::Error;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::{fmt, str::FromStr};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Callsign {
    callsign: String,
}

impl Callsign {
    pub(crate) fn new(s: &str) -> Self {
        Self {
            callsign: s.to_lowercase(),
        }
    }

    pub(crate) fn lower(&self) -> &str {
        &self.callsign
    }
}

impl fmt::Display for Callsign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.callsign.to_uppercase())
    }
}

impl FromStr for Callsign {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Callsign::new(s))
    }
}

impl<'de> Deserialize<'de> for Callsign {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CallsignVisitor;

        impl<'de> Visitor<'de> for CallsignVisitor {
            type Value = Callsign;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Callsign")
            }

            fn visit_str<E>(self, v: &str) -> Result<Callsign, E>
            where
                E: de::Error,
            {
                Ok(Callsign::new(v))
            }
        }

        deserializer.deserialize_str(CallsignVisitor)
    }
}
