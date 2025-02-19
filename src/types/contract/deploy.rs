use crate::*;

// Used to deploy aea contracts with the account-factory
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct DeployArgs {
    pub account_prefix: String,
    pub wasm_base64: String,
    pub init_method_name: String,
    pub init_args: Option<String>,
}

// Used to deploy third party dao contracts
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct DeployContractInput {
    pub account_prefix: String,
    pub wasm_hash: CryptoHash,
    pub init_method_name: String,
    pub init_args: Option<String>,
    pub initial_balance: YoctoAmount,
    pub source_code_link: Option<String>,
    pub compiler_version: Option<String>,
}