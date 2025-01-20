use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Amount {
    pub token_id: TokenContractId,
    pub amount: u128,
}