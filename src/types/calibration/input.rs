use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct CalibrationInput {
    pub group_id: GroupId,
    pub kind: CalibrationKind,
    pub cooldown_period: u64,
    pub adjustment_amount_per_weight: YoctoNumber,
}