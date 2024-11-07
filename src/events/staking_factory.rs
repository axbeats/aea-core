use crate::*;
use super::*;

// Define the event variants for staking factory events
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
pub enum StakingFactoryEventKind {
    CreateDeployStaking(CreateDeployStakingEvent),
}

// Define the main StakingFactoryEvent struct
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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

// Event for creating and deploying a staking contract
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateDeployStakingEvent {
    pub staking_id: AccountId,
    pub timestamp: u64,
}

impl CreateDeployStakingEvent {
    pub fn emit(self) {
        let event = StakingFactoryEvent::new(StakingFactoryEventKind::CreateDeployStaking(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateDeployStakingEvent {
    fn event_kind(&self) -> &str {
        "create_deploy_staking"
    }
}
