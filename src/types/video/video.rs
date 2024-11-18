use crate::*;

pub type VideoId = u64;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Video {
    pub id: VideoId,
    pub creator_id: AccountId,
    pub context: VideoContext,
    pub title: String,
    pub description: Option<String>,
    pub video: VideoHash,
    pub image: ImageHash,
    pub location: Option<String>,
    pub permissions: VideoPermissions,
    pub created_at: TimestampNanoSeconds,
    pub updated_at: Option<Vec<TimestampNanoSeconds>>,
}

impl Video {
    pub fn from_input(id: VideoId, input: VideoInput) -> Self {
        let creator_id = env::predecessor_account_id();

        Video {
            id,
            creator_id,
            context: input.context,
            title: input.title,
            description: input.description,
            video: input.video,
            image: input.image,
            location: input.location,
            permissions: input.permissions,
            created_at: env::block_timestamp_ms(),
            updated_at: None,
        }
    }
}
