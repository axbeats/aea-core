use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Profile {
    pub id: AccountId,
    pub kind: ProfileKind,
    pub name: String,
    pub bio: Option<String>,
    pub image: ImageHash,
    // pub video: Option<VideoHash>,
    pub video: Option<VideoId>,
    pub link: Option<String>,
    pub public_keys: Option<PublicKeys>,
    pub joined_date: u64,
}

impl Profile {
    pub fn from_input(account_id: AccountId, input: ProfileInput) -> Self {

        Self { 
            id: account_id, 
            kind: input.kind,
            name: input.name, 
            bio: input.bio, 
            image: input.image, 
            video: input.video, 
            link: input.link, 
            public_keys: input.public_keys, 
            joined_date: env::block_timestamp_ms()
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ProfileVideo {
    video: VideoId,
    method: GovernanceMethod,
}

impl ProfileVideo {
    pub fn new(video: VideoId) -> Self {
        Self { 
            video, 
            method: GovernanceMethod::Direct 
        }
    }
}

// #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
// #[serde(crate = "near_sdk::serde")]
// pub enum ProfileVideo {
//     Standard(VideoId),
//     Governed(Governed<VideoId>),
// }

// impl ProfileVideo {
//     pub fn video(&self) -> VideoId {
//         match self {
//             Self::Standard(video_id) => *video_id,
//             Self::Governed(gov_video) => gov_video.value,
//         }
//     }
// }