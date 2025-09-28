use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct VideoMedia {
    pub hash: VideoHash,
    pub ipfs_hash: Option<String>,
    pub streaming_url: String,
    pub file_size: u64,
    pub duration: u32,
    pub resolution: VideoResolution,
    pub thumbnail_timestamp: u32,
}

impl Default for VideoMedia {
    fn default() -> Self {
        Self {
            hash: VideoHash::default(),
            ipfs_hash: None,
            streaming_url: String::default(),
            file_size: 0,
            duration: 0,
            resolution: VideoResolution::default(),
            thumbnail_timestamp: 0,
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum VideoIdOrMedia {
    Id(VideoId),
    Media(VideoMedia),
}