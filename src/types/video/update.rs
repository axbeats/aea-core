use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum UpdateVideoAction {
    Caption(Option<String>),
    VideoMedia(VideoMedia),
    Permissions(VideoPermissions),
}