use crate::*;

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
// #[serde(crate = "near_sdk::serde")]
// pub struct ChoiceValueUpdate {
//     pub value_id: ValueId,
//     pub choice_id: ChoiceId,
//     pub value: ValueVector,
// }

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ChoiceValueUpdate {
    pub value_id: ValueId,
    pub choice_id: ChoiceId,
    pub structure: ValueStructure, // Updated to match Value struct
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceGroupUpdate {
    pub group_id: GroupId,
    pub choice_id: ChoiceId,
    pub members: Vec<AccountId>,
}
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct UpdateChoiceInput {
    pub id: ChoiceId,
    pub video_hash: Option<VideoHash>,
    pub thumbnail_hash: Option<ImageHash>,
    pub description: Option<String>,
    pub size: Option<u8>,
}