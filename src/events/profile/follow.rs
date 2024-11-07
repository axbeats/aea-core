use super::*;

// FollowProfileEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct FollowProfileEvent {
    pub follower_id: AccountId,
    pub followed_id: AccountId,
    pub timestamp: u64,
}

impl FollowProfileEvent {
    pub fn emit(self) {
        let event = ProfileEvent::new(ProfileEventKind::FollowProfile(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for FollowProfileEvent {
    fn event_kind(&self) -> &str {
        "follow_profile"
    }
}