use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum VoteMintInteraction {
    CastProposalVote,
    CastChoiceVote,
    CastCalibrationVote,
    ReceiveApproveVote,
    ReceiveRejectVote,
    ReceiveSpamVote,
}

impl VoteMintInteraction {
    pub fn total_weight(&self, weights: &VoteMintWeights) -> i64 {
        match self {
            VoteMintInteraction::CastProposalVote => weights.cast_proposal_vote as i64,
            VoteMintInteraction::CastChoiceVote => weights.cast_choice_vote as i64,
            VoteMintInteraction::CastCalibrationVote => weights.cast_calibration_vote as i64,
            VoteMintInteraction::ReceiveApproveVote => weights.receive_approve_vote as i64,
            VoteMintInteraction::ReceiveRejectVote => weights.receive_reject_vote as i64,
            VoteMintInteraction::ReceiveSpamVote => weights.receive_spam_vote as i64,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VoteMintWeights {
    pub cast_proposal_vote: MintWeight,
    pub cast_choice_vote: MintWeight,
    pub cast_calibration_vote: MintWeight,
    pub receive_approve_vote: MintWeight,
    pub receive_reject_vote: MintWeight,
    pub receive_spam_vote: MintWeight,
}
