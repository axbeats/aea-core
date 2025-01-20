use super::*;

#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct RemoveReviewEvent {
    pub review_id: ReviewId,
}

impl RemoveReviewEvent {
    pub fn emit(self) {
        let event = CourtEvent::new(CourtEventKind::RemoveReview(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for RemoveReviewEvent {
    fn event_kind(&self) -> &str {
        "remove_review"
    }
}