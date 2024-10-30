use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct BooleanValidation {
    pub must_be_true: Option<bool>, // If set, enforces that the value must be true or false
}