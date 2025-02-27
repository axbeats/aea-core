use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct AddManagedContractInput {
    pub dao_id: DaoId,
    pub contract_id: ContractId,
    pub wasm_hash: CryptoHash,
    pub source_code_link: Option<String>,
    pub compiler_version: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub video: VideoHash,
    pub image: ImageHash,
    pub location: Option<String>,
}