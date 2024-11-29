use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct DistributionType {
    pub elements: HashMap<String, DistributionPercentage>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct DistributionInput {
    pub elements: HashMap<String, DistributionPercentageInput>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct DistributionPercentage {
    pub percentage: UInt128Id,
    pub weights: Option<WeightType>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct DistributionPercentageInput {
    pub percentage: u128,
    pub weights: Option<WeightType>,
}