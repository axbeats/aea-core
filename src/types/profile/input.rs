use crate::*;
use aea_macros::Generable;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct ProfileInput {
    pub username: String,
    pub kind: ProfileKind,
    pub visibility: ProfileVisibility,
    pub name: String,
    pub bio: Option<String>,
    pub link: Option<String>,
    pub photo: ProfilePhoto,
    pub public_keys: Option<PublicKeys>,
}

impl Default for ProfileInput {
    fn default() -> Self {
        Self::example()
    }
}

impl ProfileInput {
    /// Generate an example ProfileInput for a DAO
    pub fn example() -> Self {
        Self {
            username: "example-dao".to_string(),
            kind: ProfileKind::Dao,
            visibility: ProfileVisibility::Public,
            name: "Example DAO".to_string(),
            bio: Some("A community-driven organization for collaborative decision making".to_string()),
            photo: ProfilePhoto {
                hash: "QmExampleImageHash123456789".to_string(),
                ipfs_hash: "QmExampleImageHash123456789".to_string(),
                streaming_url: "https://example-dao.near.social".to_string(),
            },
            public_keys: None,
            link: Some("https://example-dao.near.social".to_string()),
        }
    }
}