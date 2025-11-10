use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum UpdateContractAction {
    Name(String),
    Caption(Option<String>),
    VideoMedia(VideoMedia),
}