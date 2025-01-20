use crate::*;

pub type ReviewId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Review {
    pub id: ReviewId,
    pub video_id: VideoId,
    pub rule_id: RuleId,
    pub dao_id: DaoId,
    pub accused_id: AccountId,
    pub group_id: GroupId,
    pub status: ReviewStatus,
    pub initiated_at: TimestampNanoSeconds,
}
