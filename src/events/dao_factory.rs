use crate::*;
use super::*;

// Define the event variants for DAO factory events
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
pub enum DaoFactoryEventKind {
    CreateDeployDao(CreateDeployDaoEvent),
}

// Define the main DaoFactoryEvent struct
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct DaoFactoryEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: DaoFactoryEventKind,
}

impl DaoFactoryEvent {
    pub fn new(event: DaoFactoryEventKind) -> Self {
        DaoFactoryEvent {
            standard: "nep171-dao-factory".to_string(),
            version: "1.0.0".to_string(),
            event,
        }
    }
}

impl std::fmt::Display for DaoFactoryEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}

// Event for creating and deploying a DAO
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateDeployDaoEvent {
    pub dao_id: AccountId,
    pub timestamp: u64,
}

impl CreateDeployDaoEvent {
    pub fn emit(self) {
        let event = DaoFactoryEvent::new(DaoFactoryEventKind::CreateDeployDao(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateDeployDaoEvent {
    fn event_kind(&self) -> &str {
        "create_deploy_dao"
    }
}
