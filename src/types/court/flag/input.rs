use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct FlagInput {
    pub rule_id: RuleId,
    pub video_id: VideoId,
    pub account_id: AccountId,
}