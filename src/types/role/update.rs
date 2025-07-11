use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum UpdateRoleAction {
    Name(String),
    Description(String),
    VideoBundle(VideoBundle),
    CustomPolicy(ProposalAbility, Option<CustomPolicy>),
    Permission(ProposalAbility, ProposalPermission),
}