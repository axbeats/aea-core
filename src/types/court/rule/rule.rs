use crate::*;

pub type RuleId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Rule {
    pub id: RuleId,
    pub video_id: VideoId,
    pub dao_id: DaoId,
    pub flag_group_id: GroupId,
    pub jury_group_id: GroupId,
    pub required_flag_count: u64,
    pub penalty_function: Option<FunctionCall>,
    pub penalty_amount: Amount,
    pub policy: RulePolicy,
    pub initiated_at: TimestampNanoSeconds,
}

impl Rule {
    pub fn from_input(id: RuleId, video_id: VideoId, input: RuleInput) -> Self {
        Self {
            id,
            video_id,
            dao_id: input.dao_id,
            flag_group_id: input.flag_group_id,
            jury_group_id: input.jury_group_id,
            required_flag_count: input.required_flag_count,
            penalty_function: input.penalty_function,
            penalty_amount: input.penalty_amount,
            policy: input.policy,
            initiated_at: env::block_timestamp(),
        }
    }
}