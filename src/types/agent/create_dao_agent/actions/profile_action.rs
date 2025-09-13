use crate::*;

/// Actions that can be performed on the DAO profile
#[near(serializers = [json, borsh])]
#[serde(tag = "action", content = "params")]
#[derive(Debug, Clone, Generable)]
pub enum ProfileAction {
    #[guide = "Change the unique identifier for the DAO"]
    UpdateUsername { 
        #[guide = "Must be 3-32 characters, lowercase letters and numbers only"]
        new_username: String 
    },
    #[guide = "Update the display name of the DAO"]
    UpdateName { 
        #[guide = "Human-readable name, up to 50 characters"]
        new_name: String 
    },
    #[guide = "Update the description of the DAO's purpose"]
    UpdateBio { 
        #[guide = "Optional description of what the DAO does"]
        new_bio: Option<String> 
    },
}

impl Default for ProfileAction {
    fn default() -> Self {
        Self::UpdateUsername { new_username: "".to_string() }
    }
}