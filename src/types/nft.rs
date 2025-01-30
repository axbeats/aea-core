use crate::*;

pub type VideoNftId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct VideoNftInput {
    pub title: String,        
    pub description: Option<String>,           
    pub video: VideoHash,                
    pub image: ImageHash,             
    pub location: Option<String>,
    pub permissions: VideoPermissions,
    pub royalties: Option<HashMap<AccountId, PercentageU32>>,
}