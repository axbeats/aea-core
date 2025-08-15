use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct TokenRole {
    pub staking_id: StakingId,
    pub weight_formula: WeightFormula,
}

impl TokenRole {
    pub fn new(staking_id: StakingId, weight_formula: WeightFormula) -> Self {
        Self {
            staking_id,
            weight_formula,
        }
    }
    
    /// Calculate vote weight based on staked tokens
    pub fn calculate_weight(&self, staked_tokens: u128) -> u128 {
        match self.weight_formula {
            WeightFormula::Linear => staked_tokens,
            WeightFormula::Quadratic => {
                // Square root for quadratic voting
                if staked_tokens == 0 {
                    0
                } else {
                    // Simple integer square root approximation
                    let mut x = staked_tokens;
                    let mut y = (x + 1) / 2;
                    while y < x {
                        x = y;
                        y = (x + staked_tokens / x) / 2;
                    }
                    x
                }
            }
        }
    }
}

impl Default for TokenRole {
    fn default() -> Self {
        Self {
            staking_id: "staking.near".parse().unwrap(),
            weight_formula: WeightFormula::Linear,
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct TokenRoleInput {
    pub weight_formula: WeightFormula,
}

impl Default for TokenRoleInput {
    fn default() -> Self {
        Self {
            weight_formula: WeightFormula::Linear,
        }
    }
}