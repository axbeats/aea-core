use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProfileInput {
    pub username: String,
    pub kind: ProfileKind,
    pub name: String,
    pub bio: Option<String>,
    pub image: ImageHash,
    pub public_keys: Option<PublicKeys>,
    pub link: Option<String>,
}