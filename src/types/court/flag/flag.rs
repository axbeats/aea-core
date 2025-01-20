use crate::*;

pub type FlagId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Flag {
    pub id: FlagId,
    pub rule_id: RuleId,
    pub video_id: VideoId,
    pub account_id: AccountId,
}

impl Flag {
    pub fn from_input(id: FlagId, input: FlagInput) -> Self {
        Self {
            id,
            rule_id: input.rule_id,
            video_id: input.video_id,
            account_id: input.account_id,
        }
    }
}