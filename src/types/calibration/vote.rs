use near_sdk::serde_json;
// use neo4rs::Node;

use crate::*;

pub type CalibrationVoteId = u64;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CalibrationVote {
    pub id: CalibrationVoteId,
    pub calibration_id: CalibrationId,
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub vote: DeltaVote,
    pub weight: Weight,
    pub timestamp: TimestampSeconds,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct DeltaVote {
    pub increase: SubValueName,
    pub decrease: SubValueName,
}

impl Default for DeltaVote {
    fn default() -> Self {
        DeltaVote {
            increase: SubValueName::default(),
            decrease: SubValueName::default(),
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
    pub vote: DeltaVote,
}

impl CalibrationVote {
    pub fn from_input(input: CalibrationVoteInput, weight: u128) -> Self {
        Self {
            id: 0, // Will set later
            calibration_id: input.calibration_id,
            account_id: input.account_id,
            dao_id: input.dao_id,
            group_id: input.group_id,
            vote: input.vote,
            weight,
            timestamp: env::block_timestamp_ms(),
        }
    }
}