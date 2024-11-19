use crate::*;

pub type MintInteractionId = String;
pub type MintWeight = i16;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MintInteraction {
    pub id: MintInteractionId,
    pub method_name: String,
    pub contract_id: AccountId,
    pub stream_id: MintStreamId,
    pub weight: MintWeight,
}
