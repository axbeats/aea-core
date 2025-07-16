use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct RulePolicy {
    pub flag_threshold: PercentageU32,   // Flags to views
    pub flag_quorum: u64,                // Min number of flags
    pub review_threshold: PercentageU32, // Guilty to innocent
    pub review_quorum: Quorum,           // Min number of votes
}

impl Default for RulePolicy {
    fn default() -> Self {
        Self {
            flag_threshold: 50,
            flag_quorum: 1,
            review_threshold: 50,
            review_quorum: Quorum::Fixed(1),
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub enum Quorum {
    Fixed(u64),
    Percentage(Percentage),
}

impl Default for Quorum {
    fn default() -> Self {
        Self::Fixed(1)
    }
}