use super::*;

// Event for updating the status of an indictment
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UpdateIndictmentStatusEvent {
    pub indictment_id: IndictmentId,
    pub status: IndictmentStatus,
    pub timestamp: u64,
}

impl UpdateIndictmentStatusEvent {
    pub fn emit(self) {
        let event = CourtEvent::new(CourtEventKind::UpdateIndictmentStatus(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateIndictmentStatusEvent {
    fn event_kind(&self) -> &str {
        "update_indictment_status"
    }
}