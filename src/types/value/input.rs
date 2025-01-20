use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ValueInput {
    pub name: ValueName,
    pub description: String,
    pub dao_id: DaoId,
    pub structure: ValueStructure,
    pub vote_method_input: VoteMethodInput,
    pub calling_contract: ContractId,
}

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub struct GovernedValueInput {
//     pub dao_id: DaoId,
//     pub operator_id: ContractId,
//     pub value: Value,
//     pub vote_method_input: VoteMethodInput,
//     // Video fields
//     pub title: String,
//     pub description: Option<String>,
//     pub video: VideoHash,
//     pub image: ImageHash,
//     pub location: Option<String>,
// }
