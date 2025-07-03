use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum UpdateGroupAction {
    Name(String),
    Description(String),
    VideoBundle(VideoBundle),
    CustomPolicy(ProposalKind, Option<CustomPolicy>),
    Permission(ProposalKind, ProposalPermission),
}