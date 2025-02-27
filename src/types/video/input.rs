use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct VideoInput {
    pub context: VideoContext,
    pub title: String,        
    pub description: Option<String>,           
    pub video: VideoHash,                
    pub image: ImageHash,             
    pub location: Option<String>,           
    pub permissions: VideoPermissions,                
}

impl VideoInput {
    pub fn from_nft_input(input: VideoNftInput, nft_id: VideoNftId) -> Self {
        Self {
            context: VideoContext::Content { nft_id },
            title: input.title,
            description: input.description,
            video: input.video,
            image: input.image,
            location: input.location,
            permissions: input.permissions,
        }
    }
 
    pub fn from_proposal_input(input: ProposalInput, proposal_id: ProposalId) -> Self {
        Self {
            context: VideoContext::Proposal { proposal_id },
            title: input.title,
            description: input.description,
            video: input.video,
            image: input.image,
            location: input.location,
            permissions: VideoPermissions::vote_permissions(),
        }
    }
 
    pub fn from_value_init(init: ValueInit, title: String, description: Option<String>) -> Self {
        Self {
            context: VideoContext::Value { value_id: init.id },
            title,
            description,
            video: init.video,
            image: init.image,
            location: init.location,
            permissions: VideoPermissions::vote_permissions(),
        }
    }
 
    pub fn from_rule_input(input: RuleInput, rule_id: RuleId) -> Self {
        Self {
            context: VideoContext::Rule { rule_id },
            title: input.name,
            description: input.description,
            video: input.video,
            image: input.image,
            location: input.location,
            permissions: VideoPermissions::vote_permissions(),
        }
    }

    pub fn from_group_input(input: GroupInput, group_id: GroupId) -> Self {
        Self {
            context: VideoContext::Group { group_id },
            title: input.name,
            description: input.description,
            video: input.video,
            image: input.image,
            location: input.location,
            permissions: VideoPermissions::group_permissions(),
        }
    }
 }