use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RuleOutput {
    pub rule: Rule,
    // pub accusations: Vec<Accusation>,
    pub reviews: Vec<ReviewOutput>,
}
