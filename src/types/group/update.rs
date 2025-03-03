use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum GroupUpdate {
    Name(String),
    Description(String),
    VideoBundle(VideoBundle),
    CustomPolicy(CustomPolicy),
    Permissions(HashMap<ProposalKindString, ProposalPermission>),
    VoteMethod(VoteMethod),
}