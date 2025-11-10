use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum UpdateValueAction {
    Caption(Option<String>),
    VideoMedia(VideoMedia),
    VoteMethod(VoteMethod),
}