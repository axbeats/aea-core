use super::*;

// Event for removing a law
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct RemoveRuleEvent {
    pub rule_id: RuleId,
    pub timestamp: u64,
}

impl RemoveRuleEvent {
    pub fn emit(self) {
        let event = CourtEvent::new(CourtEventKind::RemoveRule(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for RemoveRuleEvent {
    fn event_kind(&self) -> &str {
        "remove_rule"
    }
}