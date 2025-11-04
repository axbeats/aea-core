use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Algorithm {
    pub balance_ratios: BalanceRatios,
    pub source_ratios: SourceRatios,
    pub weight_ratios: WeightRatios,
    pub algorithm_ratios: AlgorithmRatios,
}

impl Default for Algorithm {
    fn default() -> Self {
        Self {
            balance_ratios: BalanceRatios::default(),
            source_ratios: SourceRatios::default(),
            weight_ratios: WeightRatios::default(),
            algorithm_ratios: AlgorithmRatios::default(),
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct BalanceRatios {
    pub algorithm: Percentage,
    pub following: Percentage,
}

impl Default for BalanceRatios {
    fn default() -> Self {
        Self {
            algorithm: 50,
            following: 50,
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct SourceRatios {
    pub fun: Percentage,
    pub vote: Percentage,
    pub court: Percentage,
}

impl Default for SourceRatios {
    fn default() -> Self {
        Self {
            fun: 50,
            vote: 25,
            court: 25,
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct AlgorithmRatios {
    pub interest: Percentage,
    pub social: Percentage,
    pub trending: Percentage,
    pub freshness: Percentage,
    pub variety: Percentage,
    pub bridge: Percentage,
}

impl Default for AlgorithmRatios {
    fn default() -> Self {
        Self {
            interest: 17,
            social: 16,
            trending: 17,
            freshness: 17,
            variety: 16,
            bridge: 17,
        }
    }
}
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct WeightRatios {
    pub like: Percentage,
    pub comment: Percentage,
    pub favorite: Percentage,
    pub share: Percentage,
}

impl Default for WeightRatios {
    fn default() -> Self {
        Self {
            like: 25,
            comment: 25,
            favorite: 25,
            share: 25,
        }
    }
}