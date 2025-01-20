use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Profile {
    pub id: AccountId,
    pub kind: ProfileKind,
    pub name: String,
    pub bio: Option<String>,
    pub image: ImageHash,
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
            video: None, 
            link: input.link, 
            public_keys: input.public_keys, 
            joined_date: env::block_timestamp()
        }
    }
}