use super::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct UpdateRuleEvent {
    pub rule: Rule,
    pub timestamp: u64,
}

impl UpdateRuleEvent {
    pub fn emit(self) {
        let event = CourtEvent::new(CourtEventKind::UpdateRule(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateRuleEvent {
    fn event_kind(&self) -> &str {
        "update_rule"
    }
}