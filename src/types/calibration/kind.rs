use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum CalibrationKind {
    Single,
    Delta,
}

impl From<CalibrationVoteKind> for CalibrationKind {
    fn from(vote_kind: CalibrationVoteKind) -> Self {
        match vote_kind {
            CalibrationVoteKind::Single(_) => CalibrationKind::Single,
            CalibrationVoteKind::Delta(_) => CalibrationKind::Delta,
        }
    }
}
