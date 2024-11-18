use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VideoInteraction {
    pub viewed: bool,
    pub liked: bool,
    pub favourited: bool,
    pub commented: bool,
    pub shared: bool,
    pub collaborated: bool,
}