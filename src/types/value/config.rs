use crate::*;

// Created after the video
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ValueConfig {
    pub id: ValueId, // ContractId, FieldId, and Identifier - set by user
    pub video_id: VideoId,
    pub dao_id: DaoId,
    pub operator_id: Option<ContractId>,
    pub method_input: VoteMethodInput,
}

impl ValueConfig {
    pub fn from_input(input: ValueInput, video_id: VideoId) -> Self {
        Self {
            id: input.id,
            video_id,
            dao_id: input.dao_id,
            // group_id: input.group_id,
            operator_id: input.operator_id,
            method_input: input.method_input,
        }
    }
}