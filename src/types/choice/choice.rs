use crate::*;

pub type ChoiceId = u64;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Choice {
    pub id: ChoiceId,
    pub value_id: GovernedValueId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub kind: ChoiceKind,
    pub max_vote_options: u8,
    pub elected: Vec<String>,
    pub initiated_at: TimestampNanoSeconds,
}

impl Choice {
    pub fn from_config(id: ChoiceId, config: ChoiceConfig) -> Self {
        Choice {
            id,
            value_id: config.value_id,
            dao_id: config.dao_id,
            group_id: config.group_id,
            kind: config.kind,
            max_vote_options: config.max_vote_options,
            elected: config.initial_values,
            initiated_at: env::block_timestamp(),
        }
    }
}