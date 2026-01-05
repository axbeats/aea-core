use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct VideoInput {
    pub context: VideoContext,
    pub visibility: VideoVisibility,
    pub caption: Option<String>,
    pub location: Option<String>,
    pub media: VideoMedia,
    pub permissions: VideoPermissions,
}

impl VideoInput {
    pub fn from_nft_input(input: VideoNftInput, nft_id: VideoNftId) -> Self {
        Self {
            context: VideoContext::Content { nft_id },
            visibility: input.visibility,
            caption: input.caption,
            location: input.location,
            media: input.video_media,
            permissions: input.permissions,
        }
    }
 
    pub fn from_proposal_input(input: ProposalInput, proposal_id: ProposalId) -> Self {
        Self {
            context: VideoContext::Proposal { proposal_id },
            visibility: VideoVisibility::default(),
            caption: input.caption,
            location: None,
            media: input.video_media,
            permissions: VideoPermissions::vote_permissions(),
        }
    }
 
    pub fn from_value_input(input: ValueInput, _title: String, description: Option<String>) -> Self {
        Self {
            context: VideoContext::Value { value_id: input.id },
            visibility: VideoVisibility::default(),
            caption: description,
            location: None,
            media: input.video_media,
            permissions: VideoPermissions::vote_permissions(),
        }
    }
 
    pub fn from_rule_input(input: RuleInput, rule_id: RuleId) -> Self {
        Self {
            context: VideoContext::Rule { rule_id },
            visibility: VideoVisibility::default(),
            caption: input.caption,
            location: None,
            media: input.video_media,
            permissions: VideoPermissions::vote_permissions(),
        }
    }

    pub fn from_role_input(input: RoleInput, role_id: RoleId) -> Self {
        Self {
            context: VideoContext::Role { role_id },
            visibility: VideoVisibility::default(),
            caption: input.caption,
            location: None,
            media: input.video_media,
            permissions: VideoPermissions::role_permissions(),
        }
    }

    pub fn from_contract_input(input: ContractInput) -> Self {
        Self {
            context: VideoContext::Contract { contract_id: input.contract_id },
            visibility: VideoVisibility::default(),
            caption: input.caption,
            location: None,
            media: input.video_media,
            permissions: VideoPermissions::vote_permissions(),
        }
    }
 }