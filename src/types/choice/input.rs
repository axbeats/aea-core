use crate::*;

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
// #[serde(crate = "near_sdk::serde")]
// pub struct ChoiceInput {
//     pub dao_id: DaoId,
//     pub group_id: GroupId,
//     pub kind: ChoiceKind,
//     pub video_hash: String,
//     pub thumbnail_hash: String,
//     pub description: String,
//     pub max_vote_options: u8,
//     pub initial_values: Vec<ValueOption>,
// }

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ChoiceInput {
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub kind: ChoiceKind,
    pub video_hash: String,
    pub thumbnail_hash: String,
    pub description: String,
    pub max_vote_options: u8,
    pub initial_values: Vec<ValueType>,
}