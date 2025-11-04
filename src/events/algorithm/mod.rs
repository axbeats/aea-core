use super::*;
use crate::*;

pub use self::default::*;
pub use self::update::*;

mod default;
mod update;

// Define the event variants for profile events
#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum AlgorithmEventKind {
    UpdateAlgorithm(UpdateAlgorithmEvent),
    UpdateDefaultAlgorithm(UpdateDefaultAlgorithmEvent),
}

// Define the main ProfileEvent struct
#[near(serializers = [json])]
#[derive(Debug)]
pub struct AlgorithmEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: AlgorithmEventKind,
}

impl AlgorithmEvent {
    pub fn new(event: AlgorithmEventKind) -> Self {
        AlgorithmEvent {
            standard: EVENT_STANDARD_NAME.to_string(),
            version: EVENT_VERSION.to_string(),
            event,
        }
    }
}

impl std::fmt::Display for AlgorithmEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}