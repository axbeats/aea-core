use crate::*;

pub type RuleId = u64;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Rule {
    pub id: RuleId,
    pub video_id: VideoId,
    pub dao_id: DaoId,
    pub accusation_group_id: GroupId,
    pub jury_group_id: GroupId,
    pub required_accusation_count: u64,
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
            accusation_group_id: input.accusation_group_id,
            jury_group_id: input.jury_group_id,
            required_accusation_count: input.required_accusation_count,
            penalty_function: input.penalty_function,
            penalty_amount: input.penalty_amount,
            policy: input.policy,
            initiated_at: env::block_timestamp(),
        }
    }
}