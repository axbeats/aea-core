use super::*;

// EditProfileEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EditProfileEvent {
    pub profile: Profile,
}

impl EditProfileEvent {
    pub fn emit(self) {
        let event = ProfileEvent::new(ProfileEventKind::EditProfile(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for EditProfileEvent {
    fn event_kind(&self) -> &str {
        "edit_profile"
    }
}