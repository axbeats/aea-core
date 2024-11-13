use super::*;

// CreateCalibrationEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateCalibrationEvent {
    pub calibration: Calibration,
    pub distribution: Distribution,  // Include the initial distribution in the event
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