use super::*;

// VoteCalibrationEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct VoteCalibrationEvent {
    pub vote: CalibrationVote,
}

impl VoteCalibrationEvent {
    pub fn emit(self) {
        let event = CalibrationEvent::new(CalibrationEventKind::VoteCalibration(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for VoteCalibrationEvent {
    fn event_kind(&self) -> &str {
        "vote_calibration"
    }
}