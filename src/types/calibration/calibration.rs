use crate::*;

pub type CalibrationId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Calibration {
    pub id: CalibrationId,
    pub value_id: GovernedValueId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub kind: CalibrationKind,
    pub cooldown_period: TimestampSeconds,
    pub adjustment_amount_per_weight: YoctoNumber,
    pub initiated_at: TimestampNanoSeconds,
}

impl Calibration {
    pub fn from_config(id: CalibrationId, config: CalibrationConfig) -> Self {
        Calibration {
            id,
            value_id: config.value_id,
            dao_id: config.dao_id,
            group_id: config.group_id,
            kind: config.kind,
            cooldown_period: config.cooldown_period,
            adjustment_amount_per_weight: config.adjustment_amount_per_weight,
            initiated_at: env::block_timestamp(),
        }
    }
}
