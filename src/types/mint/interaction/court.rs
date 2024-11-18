use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum CourtMintInteraction {
    GuiltyAccusation,
    InnocentAccusation,
    CastReviewVote,
}

impl CourtMintInteraction {
    pub fn total_weight(&self, weights: &CourtMintWeights) -> i64 {
        match self {
            CourtMintInteraction::GuiltyAccusation => weights.guilty_accusation as i64,
            CourtMintInteraction::InnocentAccusation => weights.innocent_accusation as i64,
            CourtMintInteraction::CastReviewVote => weights.cast_review_vote as i64,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CourtMintWeights {
    pub guilty_accusation: MintWeight,
    pub innocent_accusation: MintWeight,
    pub cast_review_vote: MintWeight,
}
