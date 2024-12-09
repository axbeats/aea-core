use crate::*;

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
// #[serde(crate = "near_sdk::serde")]
// pub enum ChoiceKind {
//     Value(ValueId),
//     Group(GroupId),
// }

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum ChoiceKind {
    Single,
    Multiple(u8), // Number of elected items
}
