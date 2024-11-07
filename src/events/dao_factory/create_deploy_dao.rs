use super::*;

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