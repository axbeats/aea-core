use crate::*;

pub type YoctoNumber = u128;
pub const ONE_YOCTO_NUMBER: u128 = 1_000_000_000_000_000_000_000_000;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum WeightFormula {
    Linear,    // 1 vote per token
    Quadratic, // n votes per n^2 tokens
}