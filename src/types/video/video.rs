use crate::*;

pub type VideoId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Video {
    pub id: VideoId,
    pub creator_id: AccountId,
    pub context: VideoContext,
    pub visibility: VideoVisibility,
    pub permissions: VideoPermissions,
    pub caption: Option<String>,
    pub location: Option<String>,
    pub media: VideoMedia,
    pub created_at: TimestampNanoSeconds,
    pub updated_at: Option<Vec<TimestampNanoSeconds>>,
}

impl Video {
    pub fn from_input(id: VideoId, input: VideoInput) -> Self {
        let creator_id = env::signer_account_id();

        Video {
            id,
            creator_id,
            context: input.context,
            visibility: input.visibility,
            permissions: input.permissions,
            caption: input.caption,
            location: input.location,
            media: input.media,
            created_at: env::block_timestamp_ms(),
            updated_at: None,
        }
    }
}
