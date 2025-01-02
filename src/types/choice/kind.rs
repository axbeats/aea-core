use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum ChoiceKind {
    Single,
    Multiple(u8), // Number of elected items
}
