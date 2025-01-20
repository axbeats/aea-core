use super::*;

#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct CreateReviewEvent {
    pub review: Review,
}

impl CreateReviewEvent {
    pub fn emit(self) {
        let event = CourtEvent::new(CourtEventKind::CreateReview(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateReviewEvent {
    fn event_kind(&self) -> &str {
        "create_review"
    }
}