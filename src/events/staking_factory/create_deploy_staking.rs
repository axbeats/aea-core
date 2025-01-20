use super::*;

// Event for creating and deploying a staking contract
#[near(serializers = [json])]
#[derive(Debug)]
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