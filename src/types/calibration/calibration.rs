use near_sdk::collections::UnorderedMap;

use crate::*;

pub type CalibrationId = u64;
pub type MatrixId = u64;

pub type Matrix = UnorderedMap<SubValueName, YoctoNumber>;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Calibration {
    pub id: CalibrationId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub video: VideoHash,
    pub image: ImageHash,
    pub description: String,
    pub cooldown_period: u64,
    pub adjustment_factor: AdjustmentFactor,
    pub initiated_at: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CalibrationInput {
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub video: VideoHash,
    pub image: ImageHash,
    pub description: String,
}