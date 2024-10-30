use crate::*;

pub type OptionId = u64;

// Have yet to implement this - Apr 16 2024
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum OptionControl {
    Open,
    Proposal,
}

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
// #[serde(crate = "near_sdk::serde")]
// pub struct ChoiceOption {
//     pub id: OptionId,
//     pub option: ValueOption,
// }

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ChoiceOption {
    pub id: OptionId,
    pub option: ValueType,
}