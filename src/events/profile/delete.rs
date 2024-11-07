use super::*;

// DeleteProfileEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct DeleteProfileEvent {
    pub account_id: AccountId,
    pub timestamp: u64,
}

impl DeleteProfileEvent {
    pub fn emit(self) {
        let event = ProfileEvent::new(ProfileEventKind::DeleteProfile(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for DeleteProfileEvent {
    fn event_kind(&self) -> &str {
        "delete_profile"
    }
}