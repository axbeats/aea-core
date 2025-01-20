use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct GroupMembers {
    pub members: Vec<AccountId>,
    pub update_method: VoteMethod,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub enum PotentialGroupKind {
    Followers,
    Subscribers,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct GroupInfo {
    pub is_member: bool,
    pub group_size: u64,
}

// This struct is an evolution of GroupInfo, but GroupInfo also has lightweight purpose - Sep 6 2024
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct GroupMembership {
    pub members: Vec<AccountId>,
    pub group_size: u64,
    pub is_member: bool,
}