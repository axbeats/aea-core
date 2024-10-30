use crate::*;

pub type LawId = u64;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Law {
    pub id: LawId,
    pub dao_id: DaoId,
    pub name: String,
    pub video: VideoHash,
    pub image: ImageHash,
    pub description: String,
    pub accusation_group_id: GroupId,
    pub jury_group_id: GroupId,
    pub required_accusation_count: u64,
    pub penalty_function: Option<FunctionCall>,
    pub penalty_amount: Amount,
    pub policy: LawPolicy,
    pub initiated_at: TimestampMilliSeconds,
}

impl Law {
    pub fn from_input(id: LawId, input: LawInput) -> Self {
        Self {
            id,
            dao_id: input.dao_id,
            name: input.name,
            video: input.video,
            image: input.image,
            description: input.description,
            accusation_group_id: input.accusation_group_id,
            jury_group_id: input.jury_group_id,
            required_accusation_count: input.required_accusation_count,
            penalty_function: input.penalty_function,
            penalty_amount: input.penalty_amount,
            policy: input.policy,
            initiated_at: env::block_timestamp_ms(),
        }
    }
}