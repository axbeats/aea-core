use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum UpdateProfileAction {
    Name(String),
    Bio(Option<String>),
    Link(Option<String>),
    Photo(ProfilePhoto),
    VideoId(VideoId),
}