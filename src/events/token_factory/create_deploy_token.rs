use super::*;

// Event for creating and deploying a token contract
#[near(serializers = [json])]
#[derive(Debug)]
pub struct CreateDeployTokenEvent {
    pub token_id: AccountId,
    pub timestamp: u64,
}

impl CreateDeployTokenEvent {
    pub fn emit(self) {
        let event = TokenFactoryEvent::new(TokenFactoryEventKind::CreateDeployToken(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateDeployTokenEvent {
    fn event_kind(&self) -> &str {
        "create_deploy_token"
    }
}