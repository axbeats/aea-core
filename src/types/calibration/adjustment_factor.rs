use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum AdjustmentFactor {
    Percentage(Percentage),
    Fixed(u128),
}