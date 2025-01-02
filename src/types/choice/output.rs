use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ChoiceOutput {
    pub choice: Choice,
    pub vote_counts: HashMap<String, u128>,
    pub user_votes: Option<ChoiceVote>,
}