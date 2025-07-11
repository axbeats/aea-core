use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceInput {
    pub role_id: RoleId,
    pub size: ChoiceSize,
    pub max_vote_options: u8,
}