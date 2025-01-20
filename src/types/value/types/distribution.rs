// use crate::*;

// #[near(serializers = [json, borsh])]
// #[derive(Debug, Clone)]
// pub struct DistributionType {
//     pub elements: HashMap<String, DistributionPercentage>,
// }

// #[near(serializers = [json, borsh])]
// #[derive(Debug, Clone)]
// pub struct DistributionInput {
//     pub elements: HashMap<String, DistributionPercentageInput>,
// }

// #[near(serializers = [json, borsh])]
// #[derive(Debug, Clone)]
// pub struct DistributionPercentage {
//     pub percentage: UInt128Id,
//     pub weights: Option<WeightType>,
// }

// #[near(serializers = [json, borsh])]
// #[derive(Debug, Clone)]
// pub struct DistributionPercentageInput {
//     pub percentage: u128,
//     pub weights: Option<WeightType>,
// }