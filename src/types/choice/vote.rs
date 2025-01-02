use crate::*;

pub type ChoiceVoteId = u64;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ChoiceVote {
    pub id: ChoiceVoteId,
    pub choice_id: ChoiceId, 
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub voted_options: HashSet<String>,
    pub weight: u128,
    pub issued_at: u64,
    pub updated_at: Option<Vec<u64>>,
}

impl ChoiceVote {
    pub fn from_config(id: ChoiceVoteId, input: ChoiceVoteConfig) -> Self {
        Self {
            id,
            choice_id: input.choice_id,
            account_id: input.account_id,
            dao_id: input.dao_id,
            group_id: input.group_id,
            voted_options: input.voted_options.into_iter().collect(), // Convert Vec to HashSet
            weight: input.weight,
            issued_at: env::block_timestamp(),
            updated_at: None,
        }   
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ChoiceVoteConfig {
    pub choice_id: ChoiceId,
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub voted_options: Vec<String>,
    pub weight: u128,
}

impl ChoiceVoteConfig {
    pub fn from_input(input: ChoiceVoteInput, weight: u128) -> Self {
        Self {
            choice_id: input.choice_id,
            account_id: input.account_id,
            dao_id: input.dao_id,
            group_id: input.group_id,
            voted_options: input.voted_options.into_iter().collect(), // Convert Vec to HashSet
            weight,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ChoiceVoteInput {
    pub choice_id: ChoiceId,
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub voted_options: Vec<String>,
}