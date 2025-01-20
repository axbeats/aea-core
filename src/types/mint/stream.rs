use crate::*;

pub type MintStreamId = String;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct MintStream {
    pub id: MintStreamId,
    pub percentage: YoctoPercentage,
    pub interactions: HashSet<MintInteractionId>,
    pub current_weights: HashMap<AccountId, u128>,
    pub carryover_negative_weights: HashMap<AccountId, u128>,
    pub last_mint_timestamp: TimestampNanoSeconds,
}

impl MintStream {
    pub fn from_input(input: MintStreamInput) -> Self {
        Self { 
            id: input.id, 
            interactions: input.interactions, 
            percentage: input.percentage, 
            current_weights: HashMap::new(), 
            carryover_negative_weights: HashMap::new(), 
            last_mint_timestamp: 0 
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct MintStreamInput {
    pub id: MintStreamId,
    pub interactions: HashSet<MintInteractionId>,
    pub percentage: YoctoPercentage,
}

