use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct ValueInput {
    pub name: ValueName,       
    pub description: String,    
    pub dao_id: DaoId,              
    pub structure: ValueStructure,           
    pub vote_method_input: VoteMethodInput,
    pub calling_contract: ContractId,
}
