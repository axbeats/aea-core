use super::*;
use crate::*;

pub use self::create_deploy_staking::*;

mod create_deploy_staking;

// Define the event variants for staking factory events
#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum StakingFactoryEventKind {
    CreateDeployStaking(CreateDeployStakingEvent),
}

// Define the main StakingFactoryEvent struct
#[near(serializers = [json])]
#[derive(Debug)]
pub struct StakingFactoryEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: StakingFactoryEventKind,
}

impl StakingFactoryEvent {
    pub fn new(event: StakingFactoryEventKind) -> Self {
        StakingFactoryEvent {
            standard: "nep171-staking-factory".to_string(),
            version: "1.0.0".to_string(),
            event,
        }
    }
}

impl std::fmt::Display for StakingFactoryEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}