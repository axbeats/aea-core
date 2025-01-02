use near_sdk::serde_json;

use crate::*;

pub type CalibrationVoteId = u64;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CalibrationVote {
    pub id: CalibrationVoteId,
    pub calibration_id: CalibrationId,
    pub value_id: GovernedValueId,
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub kind: CalibrationVoteKind,
    pub weight: Weight,
    pub adjustment_amount: YoctoNumber,
    pub timestamp: TimestampSeconds,
}

impl CalibrationVote {
    pub fn from_config(id: CalibrationVoteId, config: CalibrationVoteConfig, value_id: GovernedValueId, adjustment_amount: YoctoNumber) -> Self {
        Self {
            id,
            calibration_id: config.calibration_id,
            value_id,
            account_id: config.account_id,
            dao_id: config.dao_id,
            group_id: config.group_id,
            kind: config.kind,
            weight: config.weight,
            adjustment_amount,
            timestamp: env::block_timestamp_ms(),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CalibrationVoteConfig {
    pub calibration_id: CalibrationId,
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub kind: CalibrationVoteKind,
    pub weight: u128,
}

impl CalibrationVoteConfig {
    pub fn from_input(input: CalibrationVoteInput, weight: u128) -> Self {
        Self {
            calibration_id: input.calibration_id,
            account_id: input.account_id,
            dao_id: input.dao_id,
            group_id: input.group_id,
            kind: input.kind,
            weight,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CalibrationVoteInput {
    pub calibration_id: CalibrationId,
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub kind: CalibrationVoteKind,
}



#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum CalibrationVoteKind {
    Single(SingleVote),
    Delta(DeltaVote),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum SingleVote {
    Increase(String),
    Decrease(String),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct DeltaVote {
    pub increase: String,
    pub decrease: String,
}

impl Default for DeltaVote {
    fn default() -> Self {
        DeltaVote {
            increase: String::default(),
            decrease: String::default(),
        }
    }
}
