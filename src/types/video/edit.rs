use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct EditVideoInput {
    pub video_id: VideoId,
    pub title: Option<String>,        
    pub description: Option<String>,                  
    pub permissions: Option<VideoPermissions>,          
}

impl EditVideoInput {
    pub fn verify_parameters(&self) {
        assert!(
            self.title.is_some() || self.description.is_some() || self.permissions.is_some(),
            "ERR_NO_EDITS"
        );
    }
}
