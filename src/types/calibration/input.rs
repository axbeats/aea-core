use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationInput {
    pub group_id: GroupId,
    pub kind: CalibrationKind,
    pub cooldown_period: u64,
    pub adjustment_amount_per_weight: YoctoNumber,
}