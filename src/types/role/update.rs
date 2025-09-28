use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum UpdateRoleAction {
    Name(String),
    Description(String),
    VideoMedia(VideoMedia),
    Permission(ProposalAbility, ProposalPermission),
}