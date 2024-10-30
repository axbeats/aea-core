use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProfileInteraction {
    pub following: bool,
    pub followed: bool,
    // pub subscribed: bool,
}