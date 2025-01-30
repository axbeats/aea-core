use crate::*;

// Input for a ProposalKind::CreateValue
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ValueInput {
    pub id: ValueId, // ContractId, FieldId, and Identifier - set by user
    pub operator_id: Option<ContractId>,
    pub method_input: VoteMethodInput,
}