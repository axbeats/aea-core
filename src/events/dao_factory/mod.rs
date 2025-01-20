use super::*;
use crate::*;

pub use self::create_deploy_dao::*;

mod create_deploy_dao;

// Define the event variants for DAO factory events
#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum DaoFactoryEventKind {
    CreateDeployDao(CreateDeployDaoEvent),
}

// Define the main DaoFactoryEvent struct
#[near(serializers = [json])]
#[derive(Debug)]
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