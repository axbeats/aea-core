use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct FactoryInfo {
    pub factory_id: AccountId,
    pub auto_update: bool,
}