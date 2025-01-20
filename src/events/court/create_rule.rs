use super::*;

#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct CreateRuleEvent {
    pub rule: Rule,
}

impl CreateRuleEvent {
    pub fn emit(self) {
        let event = CourtEvent::new(CourtEventKind::CreateRule(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateRuleEvent {
    fn event_kind(&self) -> &str {
        "create_rule"
    }
}