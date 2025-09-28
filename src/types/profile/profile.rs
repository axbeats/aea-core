use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Profile {
    pub id: AccountId,
    pub kind: ProfileKind,
    pub visibility: ProfileVisibility,
    pub name: String,
    pub bio: Option<String>,
    pub link: Option<String>,
    pub photo: ProfilePhoto,
    pub video_id: Option<VideoId>,
    pub public_keys: Option<PublicKeys>,
    pub joined_date: u64,
}

impl Profile {
    pub fn from_input(account_id: AccountId, input: ProfileInput) -> Self {

        Self { 
            id: account_id, 
            kind: input.kind,
            visibility: input.visibility,
            name: input.name, 
            bio: input.bio, 
            link: input.link, 
            photo: input.photo,
            video_id: None, 
            public_keys: input.public_keys, 
            joined_date: env::block_timestamp()
        }
    }
}