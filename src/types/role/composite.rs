use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct RoleComposite {
    pub role: Role,
    pub size: u64,
    pub members: Vec<AccountId>,
    pub is_member: bool
}