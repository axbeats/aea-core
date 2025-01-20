use super::*;

// RemoveCalibrationEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct RemoveCalibrationEvent {
    pub calibration_id: CalibrationId,
    pub timestamp: u64,
}

impl RemoveCalibrationEvent {
    pub fn emit(self) {
        let event = CalibrationEvent::new(CalibrationEventKind::RemoveCalibration(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for RemoveCalibrationEvent {
    fn event_kind(&self) -> &str {
        "remove_calibration"
    }
}