use crate::*;

// Created after a successful Proposal Vote
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ValueInit {
    pub id: GovernedValueId, // ContractId, FieldId, and Identifier - set by user
    pub dao_id: DaoId,
    pub operator_id: Option<ContractId>,
    pub method_input: VoteMethodInput,
    pub video: VideoHash,
    pub image: ImageHash,
    pub location: Option<String>,
}
