use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Bond {
    pub account_id: AccountId,
    pub amount: u128,
}

impl Default for Bond {
    fn default() -> Self {
        Self {
            account_id: "example.near".parse().unwrap(),
            amount: 0,
        }
    }
}