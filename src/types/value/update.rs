use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum UpdateValueAction {
    Name(String),
    Description(String),
    VideoMedia(VideoMedia),
    VoteMethod(VoteMethod),
}