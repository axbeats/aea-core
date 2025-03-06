use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct VideoBundle {
    pub video: VideoHash,
    pub image: ImageHash,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum VideoIdOrBundle {
    Id(VideoId),
    Bundle(VideoBundle),
}