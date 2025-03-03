use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum ProfileUpdate {
    Name(String),
    Bio(Option<String>),
    Link(Option<VideoBundle>),
    Image(ImageHash),
    Video(VideoId),
}