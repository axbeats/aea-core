use crate::*;

pub type VideoNftId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct VideoNftInput {
    pub title: String,
    pub description: Option<String>,
    pub video_media: VideoMedia,
    pub visibility: VideoVisibility,
    pub permissions: VideoPermissions,
    pub royalties: Option<HashMap<AccountId, PercentageU32>>,
    pub location: Option<String>,
}