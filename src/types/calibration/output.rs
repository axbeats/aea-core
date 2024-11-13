use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CalibrationOutput {
    pub calibration: Calibration,
    pub distribution: Distribution,
    pub user_vote_sum: Distribution,
    pub engagement: VoteEngagement,
    pub last_vote: Option<CalibrationVote>,
}