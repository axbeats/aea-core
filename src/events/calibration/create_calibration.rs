use super::*;

// CreateCalibrationEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct CreateCalibrationEvent {
    pub calibration: Calibration,
}

impl CreateCalibrationEvent {
    pub fn emit(self) {
        let event = CalibrationEvent::new(CalibrationEventKind::CreateCalibration(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateCalibrationEvent {
    fn event_kind(&self) -> &str {
        "create_calibration"
    }
}