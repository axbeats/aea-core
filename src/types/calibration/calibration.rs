use crate::*;

pub type CalibrationId = u64;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Calibration {
    pub id: CalibrationId,
    pub value_id: ValueId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub video: VideoHash,
    pub image: ImageHash,
    pub description: String,
    pub cooldown_period: TimestampSeconds,
    pub adjustment_factor: AdjustmentFactor,
    pub initiated_at: u64,
}

impl Calibration {
    pub fn from(id: CalibrationId, input: CalibrationInput) -> Self {
        Calibration {
            id,
            value_id: input.value_id,
            dao_id: input.dao_id,
            group_id: input.group_id,
            video: input.video,
            image: input.image,
            description: input.description,
            cooldown_period: input.cooldown_period,
            adjustment_factor: input.adjustment_factor,
            initiated_at: env::block_timestamp() / 1_000,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CalibrationInput {
    pub value_id: ValueId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub video: VideoHash,
    pub image: ImageHash,
    pub description: String,
    pub cooldown_period: u64,
    pub adjustment_factor: AdjustmentFactor,
}
