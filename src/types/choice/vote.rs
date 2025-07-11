use crate::*;

pub type ChoiceVoteId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ChoiceVote {
    pub id: ChoiceVoteId,
    pub choice_id: ChoiceId, 
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub role_id: RoleId,
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
            role_id: input.role_id,
            voted_options: input.voted_options.into_iter().collect(), // Convert Vec to HashSet
            weight: input.weight,
            issued_at: env::block_timestamp(),
            updated_at: None,
        }   
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ChoiceVoteConfig {
    pub choice_id: ChoiceId,
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub role_id: RoleId,
    pub voted_options: Vec<String>,
    pub weight: u128,
}

impl ChoiceVoteConfig {
    pub fn from_input(input: ChoiceVoteInput, weight: u128) -> Self {
        Self {
            choice_id: input.choice_id,
            account_id: input.account_id,
            dao_id: input.dao_id,
            role_id: input.role_id,
            voted_options: input.voted_options.into_iter().collect(), // Convert Vec to HashSet
            weight,
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ChoiceVoteInput {
    pub choice_id: ChoiceId,
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub role_id: RoleId,
    pub voted_options: Vec<String>,
    pub user_location: Option<Point>,  // User's coordinates for region verification
}