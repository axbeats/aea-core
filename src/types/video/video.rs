use crate::*;

pub type VideoId = u64;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Video {
    pub id: VideoId,
    pub owner_id: AccountId,
    pub creator_id: AccountId,
    pub title: String,
    pub caption: Option<String>,
    pub video: VideoHash,
    pub image: ImageHash,
    pub location: Option<String>,
    pub issued_at: u64, // Milliseconds since Unix epoch
    pub updated_at: Option<u64>, // Milliseconds since Unix epoch
    pub permissions: VideoPermissions,
    pub tags: Option<Vec<Tag>>,
    pub royalty: Option<HashMap<AccountId, u32>>,
}

impl Video {
    pub fn from_input(id: VideoId, input: VideoInput) -> Self { 

        let account_id = input.owner_id.unwrap_or_else(|| env::predecessor_account_id());

        let permissions = VideoPermissions { 
            allow_comments: input.allow_comments, 
            allow_collaborations: input.allow_collaborations, 
            allow_promoted: input.allow_promoted, 
            approved_account_ids: Default::default(), 
            next_approval_id: 0, 
        };

        Video { 
            id, 
            owner_id: account_id.clone(), 
            creator_id: account_id, 
            title: input.title, 
            caption: input.caption, 
            video: input.video, 
            image: input.image, 
            location: input.location, 
            issued_at: env::block_timestamp_ms(), 
            updated_at: None, 
            permissions, 
            tags: input.tags, 
            royalty: input.royalties, 
        }
    }
}