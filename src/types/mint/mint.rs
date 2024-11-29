use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MintAmount {
    pub account_id: AccountId,
    pub amount: NearToken,
}