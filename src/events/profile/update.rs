use super::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct UpdateProfileEvent {
    pub profile: Profile,
    pub timestamp: u64,
}

impl UpdateProfileEvent {
    pub fn emit(self) {
        let event = ProfileEvent::new(ProfileEventKind::UpdateProfile(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateProfileEvent {
    fn event_kind(&self) -> &str {
        "update_profile"
    }
}