use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LawOutput {
    pub law: Law,
    pub accusations: Vec<Accusation>,
    pub indictments: Vec<IndictmentOutput>,
}
