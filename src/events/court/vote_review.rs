use super::*;

#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct VoteReviewEvent {
    pub vote: ReviewVote,
}

impl VoteReviewEvent {
    pub fn emit(self) {
        let event = CourtEvent::new(CourtEventKind::VoteReview(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for VoteReviewEvent {
    fn event_kind(&self) -> &str {
        "vote_review"
    }
}