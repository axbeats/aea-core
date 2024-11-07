use super::*;

// UnfollowProfileEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UnfollowProfileEvent {
    pub follower_id: AccountId,
    pub followed_id: AccountId,
    pub timestamp: u64,
}

impl UnfollowProfileEvent {
    pub fn emit(self) {
        let event = ProfileEvent::new(ProfileEventKind::UnfollowProfile(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UnfollowProfileEvent {
    fn event_kind(&self) -> &str {
        "unfollow_profile"
    }
}