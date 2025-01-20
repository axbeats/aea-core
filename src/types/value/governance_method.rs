use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum GovernanceMethod {
    Direct,
    Choice(ChoiceId),
    Calibration(CalibrationId),
}
