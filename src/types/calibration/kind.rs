use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
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
