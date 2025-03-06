use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum UpdateVideoAction {
    Title(String),
    Description(Option<String>),
    VideoBundle(VideoBundle),
    Permissions(VideoPermissions),
}