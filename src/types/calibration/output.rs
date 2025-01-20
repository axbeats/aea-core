use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct CalibrationOutput {
    pub calibration: Calibration,
    // pub distribution: Distribution,
    // pub user_vote_sum: Distribution,
    pub engagement: VoteEngagement,
    pub last_vote: Option<CalibrationVote>,
}