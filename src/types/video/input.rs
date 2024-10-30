use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VideoInput {
    pub owner_id: Option<AccountId>,
    pub title: String,        
    pub caption: Option<String>,           
    pub video: String,                
    pub image: String,             
    pub location: Option<String>,           
    pub allow_comments: bool,                 
    pub allow_collaborations: bool,        
    pub allow_promoted: bool,                
    pub tags: Option<Vec<Tag>>,             
    pub royalties: Option<HashMap<AccountId, u32>>, 
}