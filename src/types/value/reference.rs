use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ValueReference {
    pub contract_id: ContractId,
    pub identifier: ValueIdentifier,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ValueIdentifier {
    AccountId(AccountId),
    Video(VideoId),
    MintStream,
    MintInteractions,
}