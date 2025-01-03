use crate::*;

pub type AlgorithmId = u64;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Algorithm {
    pub id: AlgorithmId,
    pub name: String,
    pub stream_distribution: StreamDistribution,
    pub focus_distribution: FocusDistribution,
    pub score_distribution: ScoreDistribution,
    pub weight_distribution: WeightDistribution,
}

impl Algorithm {
    pub fn from_input(id: AlgorithmId, input: AlgorithmInput) -> Self {
        Self {
            id,
            name: input.name,
            stream_distribution: input.stream_distribution,
            focus_distribution: input.focus_distribution,
            score_distribution: input.score_distribution,
            weight_distribution: input.weight_distribution,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AlgorithmInput {
    pub name: String,
    pub stream_distribution: StreamDistribution,
    pub focus_distribution: FocusDistribution,
    pub score_distribution: ScoreDistribution,
    pub weight_distribution: WeightDistribution,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct StreamDistribution {
    pub recommended: Percentage,
    pub following: Percentage,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FocusDistribution {
    pub entertainment: Percentage,
    pub governance: Percentage,
    pub court: Percentage,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ScoreDistribution {
    pub interest: Percentage,
    pub social: Percentage,
    pub trending: Percentage,
    pub fresh: Percentage,
    pub diversity: Percentage,
    pub bridge: Percentage,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct WeightDistribution {
    pub view: Percentage,
    pub like: Percentage,
    pub favorite: Percentage,
    pub comment: Percentage,
    pub share: Percentage,
}
