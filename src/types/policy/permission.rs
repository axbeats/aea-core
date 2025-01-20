use crate::*;

pub type PermissionString = String;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProposalPermission {
    pub create: bool,
    pub vote: bool,
    pub order: Option<VoteOrder>,
    pub custom_policy: Option<CustomPolicy>,
}