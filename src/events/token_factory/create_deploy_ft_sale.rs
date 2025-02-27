use super::*;

// Event for creating and deploying a token contract
#[near(serializers = [json])]
#[derive(Debug)]
pub struct CreateFTSaleEvent {
    pub ft_sale_id: AccountId,
    pub timestamp: u64,
}

impl CreateFTSaleEvent {
    pub fn emit(self) {
        let event = TokenFactoryEvent::new(TokenFactoryEventKind::CreateFTSale(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateFTSaleEvent {
    fn event_kind(&self) -> &str {
        "create_deploy_ft_sale"
    }
}