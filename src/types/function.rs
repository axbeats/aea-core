use crate::*;

/// Function call arguments.
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct FunctionCall {
    pub contract_id: AccountId,
    pub method_name: String,
    pub args: Base64VecU8,
    pub deposit: U128,
    pub gas: U64,
}

impl Default for FunctionCall {
    fn default() -> Self {
        Self {
            contract_id: "example.near".parse().unwrap(),
            method_name: "example_method".to_string(),
            args: Base64VecU8::from(vec![]),
            deposit: U128::from(0),
            gas: U64::from(30_000_000_000_000), // 30 TGas
        }
    }
}