use crate::*;

pub type VideoNFTId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct VideoNFTInput {
    pub title: String,        
    pub description: Option<String>,           
    pub video: VideoHash,                
    pub image: ImageHash,             
    pub location: Option<String>,
    pub royalties: Option<HashMap<AccountId, PercentageU32>>,
}