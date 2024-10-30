use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProfileInput {
    pub username: String,
    pub kind: ProfileKind,
    pub name: String,
    pub bio: Option<String>,
    pub image: ImageHash,
    pub video: Option<VideoHash>,
    pub public_keys: Option<PublicKeys>,
    pub link: Option<String>,
}