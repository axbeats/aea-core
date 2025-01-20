use super::*;
use crate::*;

pub use self::create::*;
pub use self::edit::*;
pub use self::delete::*;
pub use self::follow::*;
pub use self::unfollow::*;

mod create;
mod edit;
mod delete;
mod follow;
mod unfollow;

// Define the event variants for profile events
#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum ProfileEventKind {
    CreateProfile(CreateProfileEvent),
    EditProfile(EditProfileEvent),
    DeleteProfile(DeleteProfileEvent),
    FollowProfile(FollowProfileEvent),
    UnfollowProfile(UnfollowProfileEvent),
}

// Define the main ProfileEvent struct
#[near(serializers = [json])]
#[derive(Debug)]
pub struct ProfileEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: ProfileEventKind,
}

impl ProfileEvent {
    pub fn new(event: ProfileEventKind) -> Self {
        ProfileEvent {
            standard: EVENT_STANDARD_NAME.to_string(),
            version: EVENT_VERSION.to_string(),
            event,
        }
    }
}

impl std::fmt::Display for ProfileEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}