use super::*;

// UpdateCalibrationEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct UpdateCalibrationEvent {
    pub calibration: Calibration,
    pub timestamp: u64,
}

impl UpdateCalibrationEvent {
    pub fn emit(self) {
        let event = CalibrationEvent::new(CalibrationEventKind::UpdateCalibration(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateCalibrationEvent {
    fn event_kind(&self) -> &str {
        "update_calibration"
    }
}