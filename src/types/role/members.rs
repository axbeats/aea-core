use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct RoleMembers {
    pub members: Vec<AccountId>,
    pub update_method: VoteMethod,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub enum PotentialRoleKind {
    Followers,
    Subscribers,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct RoleInfo {
    pub is_member: bool,
    pub role_size: u64,
}

// This struct is an evolution of RoleInfo, but RoleInfo also has lightweight purpose - Sep 6 2024
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct RoleMembership {
    pub members: Vec<AccountId>,
    pub role_size: u64,
    pub is_member: bool,
}