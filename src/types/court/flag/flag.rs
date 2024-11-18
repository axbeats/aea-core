use crate::*;

pub type FlagId = u64;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Flag {
    pub rule_id: RuleId,
    pub video_id: VideoId,
    pub account_id: AccountId,
}