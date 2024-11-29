use crate::*;

pub type CalibrationId = u64;

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub struct Calibration {
//     pub id: CalibrationId,
//     pub video_id: VideoId,
//     pub value_id: ValueId,
//     pub dao_id: DaoId,
//     pub group_id: GroupId,
//     pub cooldown_period: TimestampSeconds,
//     pub adjustment_factor: AdjustmentFactor,
//     pub initiated_at: TimestampNanoSeconds,
// }

// impl Calibration {
//     pub fn from_input(id: CalibrationId, video_id: VideoId, input: CalibrationInput) -> Self {
//         Calibration {
//             id,
//             video_id,
//             value_id: input.value_id,
//             dao_id: input.dao_id,
//             group_id: input.group_id,
//             cooldown_period: input.cooldown_period,
//             adjustment_factor: input.adjustment_factor,
//             initiated_at: env::block_timestamp(),
//         }
//     }
// }

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Calibration {
    pub id: CalibrationId,
    pub video_id: VideoId,
    pub reference: ValueReference,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub kind: CalibrationKind,
    pub cooldown_period: TimestampSeconds,
    pub adjustment_factor: AdjustmentFactor,
    pub initiated_at: TimestampNanoSeconds,
}

impl Calibration {
    pub fn from_input(id: CalibrationId, video_id: VideoId, input: CalibrationInput) -> Self {
        Calibration {
            id,
            video_id,
            reference: input.reference,
            dao_id: input.dao_id,
            group_id: input.group_id,
            kind: input.kind,
            cooldown_period: input.cooldown_period,
            adjustment_factor: input.adjustment_factor,
            initiated_at: env::block_timestamp(),
        }
    }
}
