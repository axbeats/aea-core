use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum UpdateRoleAction {
    Name(String),
    Caption(String),
    VideoMedia(VideoMedia),
    Permission(ProposalAbility, ProposalPermission),
}