use super::*;

// CreateProfileEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateProfileEvent {
    pub profile: Profile, // Assuming you have a Profile struct
}

impl CreateProfileEvent {
    pub fn emit(self) {
        let event = ProfileEvent::new(ProfileEventKind::CreateProfile(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateProfileEvent {
    fn event_kind(&self) -> &str {
        "create_profile"
    }
}