use super::*;
use crate::*;

pub use self::function_call_executed::*;
pub use self::transfer_executed::*;

mod function_call_executed;
mod transfer_executed;

/// Event variants for DAO execution events
#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum DaoEventKind {
    FunctionCallExecuted(FunctionCallExecutedEvent),
    TransferExecuted(TransferExecutedEvent),
}

/// Main DaoEvent struct wrapping all DAO events
#[near(serializers = [json])]
#[derive(Debug)]
pub struct DaoEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: DaoEventKind,
}

impl DaoEvent {
    pub fn new(event: DaoEventKind) -> Self {
        DaoEvent {
            standard: "nep171-dao".to_string(),
            version: "1.0.0".to_string(),
            event,
        }
    }
}

impl std::fmt::Display for DaoEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}
