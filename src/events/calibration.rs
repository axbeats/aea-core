use crate::*;
use super::*;

// Define the event variants for calibration events
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
pub enum CalibrationEventKind {
    CreateCalibration(CreateCalibrationEvent),
    UpdateCalibration(UpdateCalibrationEvent),
    RemoveCalibration(RemoveCalibrationEvent),
    VoteCalibration(VoteCalibrationEvent),
}

// Define the main CalibrationEvent struct
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CalibrationEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: CalibrationEventKind,
}

impl CalibrationEvent {
    pub fn new(event: CalibrationEventKind) -> Self {
        CalibrationEvent {
            standard: "nep171-calibration".to_string(),
            version: "1.0.0".to_string(),
            event,
        }
    }
}

impl std::fmt::Display for CalibrationEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}

// CreateCalibrationEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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

// UpdateCalibrationEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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

// RemoveCalibrationEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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

// VoteCalibrationEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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
