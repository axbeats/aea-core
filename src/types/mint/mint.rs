use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct MintAmount {
    pub account_id: AccountId,
    pub amount: NearToken,
}