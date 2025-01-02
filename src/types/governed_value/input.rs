use crate::*;

// Input for a ProposalKind::CreateValue
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct GovernedValueInput {
    pub id: GovernedValueId, // ContractId, FieldId, and Identifier - set by user
    pub operator_id: Option<ContractId>,
    pub method_input: VoteMethodInput,
}