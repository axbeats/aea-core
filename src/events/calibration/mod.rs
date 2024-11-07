use super::*;
use crate::*;

pub use self::create_calibration::*;
pub use self::remove_calibration::*;
pub use self::update_calibration::*;
pub use self::vote_calibration::*;

mod create_calibration;
mod remove_calibration;
mod update_calibration;
mod vote_calibration;

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