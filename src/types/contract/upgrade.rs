use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct UpgradeContractInput {
    pub dao_id: DaoId,
    pub contract_id: ContractId,
    pub method_name: String,
    pub wasm_hash: CryptoHash,
    pub source_link: Option<String>,
    pub compiler_version: Option<String>,
}