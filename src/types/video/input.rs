use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
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
    pub fn from_proposal_input(input: ProposalInput, proposal_id: ProposalId) -> Self {
        Self {
            context: VideoContext::Proposal(proposal_id),
            title: input.title,
            description: input.description,
            video: input.video,
            image: input.image,
            location: input.location,
            permissions: VideoPermissions::vote_permissions(),
        }
    }
}

impl VideoInput {
    pub fn from_choice_input(input: ChoiceInput, choice_id: ChoiceId) -> Self {
        Self {
            context: VideoContext::Choice(choice_id),
            title: input.title,
            description: input.description,
            video: input.video,
            image: input.image,
            location: input.location,
            permissions: VideoPermissions::vote_permissions(),
        }
    }
}

impl VideoInput {
    pub fn from_calibration_input(input: CalibrationInput, calibration_id: CalibrationId) -> Self {
        Self {
            context: VideoContext::Calibration(calibration_id),
            title: input.title,
            description: input.description,
            video: input.video,
            image: input.image,
            location: input.location,
            permissions: VideoPermissions::vote_permissions(),
        }
    }
}

impl VideoInput {
    pub fn from_rule_input(input: RuleInput, rule_id: RuleId) -> Self {
        Self {
            context: VideoContext::Rule(rule_id),
            title: input.title,
            description: input.description,
            video: input.video,
            image: input.image,
            location: input.location,
            permissions: VideoPermissions::vote_permissions(),
        }
    }
}