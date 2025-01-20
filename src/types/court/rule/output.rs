use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct RuleOutput {
    pub rule: Rule,
    // pub accusations: Vec<Accusation>,
    pub reviews: Vec<ReviewOutput>,
}
