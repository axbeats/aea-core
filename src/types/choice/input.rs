use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct ChoiceInput {
    pub group_id: GroupId,
    pub kind: ChoiceKind,
    pub max_vote_options: u8,
}