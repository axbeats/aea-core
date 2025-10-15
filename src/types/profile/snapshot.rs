use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProfileSnapshot {
    pub id: AccountId,
    pub name: String,
    pub photo: ProfilePhoto,
}