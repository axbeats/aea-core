use crate::*;
use near_sdk::json_types::U128;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct Amount {
    pub token_id: TokenContractId,
    pub amount: U128,
}

impl Default for Amount {
    fn default() -> Self {
        Self {
            token_id: "example.near".parse().unwrap(),
            amount: U128(0),
        }
    }
}