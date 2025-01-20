use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct FactoryInfo {
    pub factory_id: AccountId,
    pub auto_update: bool,
}