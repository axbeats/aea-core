use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct CalibrationConfig {
    pub value_id: GovernedValueId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub kind: CalibrationKind,
    pub cooldown_period: u64,
    pub adjustment_amount_per_weight: YoctoNumber,
}

// I think this method can be simplified - Dec 30 2024
impl From<ValueConfig> for CalibrationConfig {
    fn from(config: ValueConfig) -> Self {
        let (group_id, kind, cooldown_period, adjustment_amount) = match config.method_input.clone() {
            VoteMethodInput::Calibration(calibration_input) => {
                (
                    calibration_input.group_id,
                    calibration_input.kind,
                    calibration_input.cooldown_period,
                    calibration_input.adjustment_amount_per_weight,
                )
            }
            _ => panic!("ValueConfig must have VoteMethodInput::Calibration to convert into CalibrationConfig"),
        };

        Self {
            value_id: config.id,
            dao_id: config.dao_id,
            group_id,
            kind,
            cooldown_period,
            adjustment_amount_per_weight: config.method_input.as_calibration().unwrap().adjustment_amount_per_weight,
        }
    }
}
