use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CalibrationReference {
    pub contract_id: ContractId,
    pub identifier: CalibrationIdentifier,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum CalibrationIdentifier {
    AccountId(AccountId),
    Video(VideoId),
    MintStream,
    MintInteractions,
}