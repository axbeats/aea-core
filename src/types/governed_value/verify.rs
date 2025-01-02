use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VerifyValueResult {
    pub title: String,
    pub description: Option<String>,
}
