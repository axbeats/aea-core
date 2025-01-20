use crate::*;

// #[near(serializers = [json, borsh])]
// #[derive(Debug, Clone)]
// pub struct WeightType {
//     pub elements: HashMap<String, UInt128Id>,
// }

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct WeightInput {
    pub elements: HashMap<String, u128>,
}