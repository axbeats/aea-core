use crate::*;

// Created after the video
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ValueConfig {
    pub id: GovernedValueId, // ContractId, FieldId, and Identifier - set by user
    pub video_id: VideoId,
    pub dao_id: DaoId,
    pub operator_id: Option<ContractId>,
    pub method_input: VoteMethodInput,
}

impl ValueConfig {
    pub fn from_init(init: ValueInit, video_id: VideoId) -> Self {
        Self {
            id: init.id,
            video_id,
            dao_id: init.dao_id,
            // group_id: init.group_id,
            operator_id: init.operator_id,
            method_input: init.method_input,
        }
    }
}