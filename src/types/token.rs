use crate::*;

pub type TokenId = u64;
pub type ApprovalId = u32;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum TokenWeightKind {
    Linear,    // 1 vote per token
    Quadratic, // n votes per n^2 tokens
}

impl TokenWeightKind {
    pub fn convert_to_weight_kind(&self) -> WeightKind {
        match self {
            TokenWeightKind::Linear => WeightKind::LinearToken,
            TokenWeightKind::Quadratic => WeightKind::QuadraticToken,
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct TransferInput {
    pub token_id: Option<AccountId>, // None for $Near
    pub receiver_id: AccountId,
    pub amount: U128,
    pub msg: Option<String>,
}