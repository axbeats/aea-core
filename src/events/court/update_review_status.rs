use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UpdateReviewStatusEvent {
    pub review_id: ReviewId,
    pub status: ReviewStatus,
    pub timestamp: u64,
}

impl UpdateReviewStatusEvent {
    pub fn emit(self) {
        let event = CourtEvent::new(CourtEventKind::UpdateReviewStatus(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateReviewStatusEvent {
    fn event_kind(&self) -> &str {
        "update_review_status"
    }
}