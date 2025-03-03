use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum ValueUpdate {
    Name(String),
    Description(String),
    VideoBundle(VideoBundle),
    Content(String), // Can this also be a vec? - Feb 28 2025
    VoteMethod(VoteMethod),
}