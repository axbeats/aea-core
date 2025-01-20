use super::*;
use crate::*;

pub use self::stake::*;
pub use self::unstake::*;

mod stake;
mod unstake;

// Define the event variants for staking events
#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum StakingEventKind {
    Stake(StakeEvent),
    Unstake(UnstakeEvent),
}

// Define the main StakingEvent struct
#[near(serializers = [json])]
#[derive(Debug)]
pub struct StakingEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: StakingEventKind,
}

impl StakingEvent {
    pub fn new(event: StakingEventKind) -> Self {
        StakingEvent {
            standard: "nep171-staking".to_string(),
            version: "1.0.0".to_string(),
            event,
        }
    }
}

impl std::fmt::Display for StakingEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}