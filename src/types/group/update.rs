use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum UpdateGroupAction {
    Name(String),
    Description(String),
    VideoBundle(VideoBundle),
    CustomPolicy(ProposalKindString, Option<CustomPolicy>),
    Permission(ProposalKindString, ProposalPermission),
    VoteMethod(VoteMethodInput),
}