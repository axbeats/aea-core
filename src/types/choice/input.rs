use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceInput {
    pub group_id: GroupId,
    pub kind: ChoiceKind,
    pub max_vote_options: u8,
}