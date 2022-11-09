use kagiyama::Watcher;
use lazy_static::lazy_static;
use matrix_sdk::ruma::UserId;
use prometheus_client::encoding::text::Encode;
use prometheus_client::metrics::{counter::Counter, family::Family};
use serde::Serialize;
use strum_macros::EnumIter;

#[derive(Clone, Serialize, EnumIter, PartialEq, Hash, Eq)]
pub(crate) enum ReadinessConditions {
    LoggedIntoMatrix,
}

#[derive(Clone, Eq, Hash, PartialEq, Encode)]
pub(crate) struct CommandLables {
    sender: String,
    kind: CommandKind,
}

impl CommandLables {
    pub(crate) fn new(sender: &UserId, kind: CommandKind) -> Self {
        Self {
            sender: sender.to_string(),
            kind,
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq, Encode)]
pub(crate) enum CommandKind {
    List,
    Get,
    Stats,
    Send,
    SendNews,
}

lazy_static! {
    pub(crate) static ref COMMANDS: Family::<CommandLables, Counter> =
        Family::<CommandLables, Counter>::default();
    pub(crate) static ref FAILURES: Counter = Counter::default();
}

pub(crate) fn register(watcher: &Watcher<ReadinessConditions>) {
    let mut registry = watcher.metrics_registry();

    {
        let registry = registry.sub_registry_with_prefix("matrixdapnetbot");
        registry.register("commands", "Command requests", Box::new(COMMANDS.clone()));
        registry.register(
            "failures",
            "Command parsing or execution failures",
            Box::new(FAILURES.clone()),
        );
    }
}
