use crate::*;

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
// #[serde(crate = "near_sdk::serde")]
// pub struct ChoiceValueUpdate {
//     pub value_id: ValueId,
//     pub choice_id: ChoiceId,
//     pub value: ValueVector,
// }

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ChoiceValueUpdate {
    pub value_id: ValueId,
    pub choice_id: ChoiceId,
    pub structure: ValueStructure, // Updated to match Value struct
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct ChoiceGroupUpdate {
    pub group_id: GroupId,
    pub choice_id: ChoiceId,
    pub members: Vec<AccountId>,
}
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UpdateChoiceInput {
    pub id: ChoiceId,
    pub video_hash: Option<VideoHash>,
    pub thumbnail_hash: Option<ImageHash>,
    pub description: Option<String>,
    pub size: Option<u8>,
}