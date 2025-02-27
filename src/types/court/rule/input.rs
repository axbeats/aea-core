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
    pub video_bundle: VideoBundle,
    pub location: Option<String>,
}
