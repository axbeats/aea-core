use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum GovernanceMethod {
    Direct,
    Choice(ChoiceId),
    Calibration(CalibrationId),
}
