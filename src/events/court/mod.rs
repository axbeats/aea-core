use super::*;
use crate::*;

pub use self::add_flag::*;
pub use self::create_review::*;
pub use self::create_rule::*;
pub use self::remove_flag::*;
pub use self::remove_review::*;
pub use self::remove_rule::*;
pub use self::update_review_status::*;
pub use self::update_rule::*;
pub use self::vote_review::*;

mod add_flag;
mod create_review;
mod create_rule;
mod remove_flag;
mod remove_review;
mod remove_rule;
mod update_review_status;
mod update_rule;
mod vote_review;

// Define the event variants for court events
#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum CourtEventKind {
    AddFlag(AddFlagEvent),
    CreateRule(CreateRuleEvent),
    RemoveFlag(RemoveFlagEvent),
    RemoveReview(RemoveReviewEvent),
    RemoveRule(RemoveRuleEvent),
    CreateReview(CreateReviewEvent),
    VoteReview(VoteReviewEvent),
    UpdateReviewStatus(UpdateReviewStatusEvent),
    UpdateRule(UpdateRuleEvent),
}

#[near(serializers = [json])]
#[derive(Debug)]
pub struct CourtEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: CourtEventKind,
}

impl CourtEvent {
    pub fn new(event: CourtEventKind) -> Self {
        CourtEvent {
            standard: EVENT_STANDARD_NAME.to_string(),
            version: EVENT_VERSION.to_string(),
            event,
        }
    }
}

impl std::fmt::Display for CourtEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}