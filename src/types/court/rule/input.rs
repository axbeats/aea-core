use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct RuleInput {
    pub dao_id: DaoId,
    pub flag_group_id: GroupId,
    pub jury_group_id: GroupId,
    pub required_flag_count: u64,
    pub penalty_function: Option<FunctionCall>,
    pub penalty_amount: Amount,
    pub policy: RulePolicy,
    // Video fields
    pub name: String,
    pub description: Option<String>,
    pub video: VideoHash,
    pub image: ImageHash,
    pub location: Option<String>,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct RuleInputVideoOption {
    pub dao_id: DaoId,
    pub flag_group_id: GroupId,
    pub jury_group_id: GroupId,
    pub required_flag_count: u64,
    pub penalty_function: Option<FunctionCall>,
    pub penalty_amount: Amount,
    pub policy: RulePolicy,
    // Video fields
    pub name: String,
    pub description: Option<String>,
    pub video_bundle: Option<VideoBundle>,
    pub location: Option<String>,
}

impl RuleInput {

    pub fn from_video_option(
        input: RuleInputVideoOption,
        proposal_video: VideoHash,
        proposal_image: ImageHash,
    ) -> Self {
        let (video, image) = if let Some(bundle) = input.video_bundle {
            (bundle.video, bundle.image)
        } else {
            (proposal_video, proposal_image)
        };
        Self {
            dao_id: input.dao_id,
            flag_group_id: input.flag_group_id,
            jury_group_id: input.jury_group_id,
            required_flag_count: input.required_flag_count,
            penalty_function: input.penalty_function,
            penalty_amount: input.penalty_amount,
            policy: input.policy,
            name: input.name,
            description: input.description,
            video,
            image,
            location: input.location,
        }
    }

    pub fn unwrap_video_option(input: RuleInputVideoOption) -> Self {

        let video_bundle = input.video_bundle.unwrap();

        Self {
            dao_id: input.dao_id,
            flag_group_id: input.flag_group_id,
            jury_group_id: input.jury_group_id,
            required_flag_count: input.required_flag_count,
            penalty_function: input.penalty_function,
            penalty_amount: input.penalty_amount,
            policy: input.policy,
            name: input.name,
            description: input.description,
            video: video_bundle.video,
            image: video_bundle.image,
            location: None,
        }
    }
}
