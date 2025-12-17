use crate::*;

// TODO: Add privacy update - Dec 2 2025
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum UpdateProfileAction {
    Name(String),
    Bio(Option<String>),
    Link(Option<String>),
    Photo(ProfilePhoto),
    VideoId(Option<VideoId>),
}