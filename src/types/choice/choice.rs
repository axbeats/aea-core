use crate::*;

pub type ChoiceId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Choice {
    pub id: ChoiceId,
    pub value_id: ValueId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub size: ChoiceSize,
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
            size: config.kind,
            max_vote_options: config.max_vote_options,
            elected: config.initial_values,
            initiated_at: env::block_timestamp(),
        }
    }
}