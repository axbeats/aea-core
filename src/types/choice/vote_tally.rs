use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceVoteTally {
    pub candidate: String,
    pub count: u128,  // Adjust if needed for your use case
}