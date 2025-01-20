use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct MutualConnections {
    pub followers: Vec<AccountId>,
    pub following: Vec<AccountId>,
}