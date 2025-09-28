use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct ProfilePhoto {
    pub hash: ImageHash,
    pub ipfs_hash: String,
    pub streaming_url: String,
}

impl Default for ProfilePhoto {
    fn default() -> Self {
        Self {
            hash: ImageHash::default(),
            ipfs_hash: "".to_string(),
            streaming_url: "".to_string(),
        }
    }
}