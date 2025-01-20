use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ChoiceOutput {
    pub choice: Choice,
    pub vote_counts: HashMap<String, u128>,
    pub user_votes: Option<ChoiceVote>,
}