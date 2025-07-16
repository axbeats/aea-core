use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct Amount {
    pub token_id: TokenContractId,
    pub amount: u128,
}

impl Default for Amount {
    fn default() -> Self {
        Self {
            token_id: "example.near".parse().unwrap(),
            amount: 0,
        }
    }
}