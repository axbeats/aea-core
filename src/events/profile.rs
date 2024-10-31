use near_sdk::{env, serde::{Deserialize, Serialize}, serde_json};
use crate::*;

// Define the event variants for profile events
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
pub enum ProfileEventLogVariant {
    CreateProfile(CreateProfileLog),
    EditProfile(EditProfileLog),
    DeleteProfile(DeleteProfileLog),
    FollowProfile(FollowProfileLog),
    UnfollowProfile(UnfollowProfileLog),
}

// Define the main ProfileEventLog struct
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ProfileEventLog {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: ProfileEventLogVariant,
}

impl std::fmt::Display for ProfileEventLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}

// Define separate structs for each action with necessary fields

// CreateProfileLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateProfileLog {
    pub profile: Profile, // Assuming you have a Profile struct
}

impl CreateProfileLog {
    pub fn emit(self) {
        let event = ProfileEventLog {
            standard: "nep171-profile".to_string(), // Use an appropriate standard name
            version: "1.0.0".to_string(),
            event: ProfileEventLogVariant::CreateProfile(self),
        };
        env::log_str(&event.to_string());
    }
}

// EditProfileLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EditProfileLog {
    pub profile: Profile,
}

impl EditProfileLog {
    pub fn emit(self) {
        let event = ProfileEventLog {
            standard: "nep171-profile".to_string(),
            version: "1.0.0".to_string(),
            event: ProfileEventLogVariant::EditProfile(self),
        };
        env::log_str(&event.to_string());
    }
}

// DeleteProfileLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct DeleteProfileLog {
    pub account_id: AccountId,
    pub timestamp: u64,
}

impl DeleteProfileLog {
    pub fn emit(self) {
        let event = ProfileEventLog {
            standard: "nep171-profile".to_string(),
            version: "1.0.0".to_string(),
            event: ProfileEventLogVariant::DeleteProfile(self),
        };
        env::log_str(&event.to_string());
    }
}

// FollowProfileLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct FollowProfileLog {
    pub follower_id: AccountId,
    pub followed_id: AccountId,
    pub timestamp: u64,
}

impl FollowProfileLog {
    pub fn emit(self) {
        let event = ProfileEventLog {
            standard: "nep171-profile".to_string(),
            version: "1.0.0".to_string(),
            event: ProfileEventLogVariant::FollowProfile(self),
        };
        env::log_str(&event.to_string());
    }
}

// UnfollowProfileLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UnfollowProfileLog {
    pub follower_id: AccountId,
    pub followed_id: AccountId,
    pub timestamp: u64,
}

impl UnfollowProfileLog {
    pub fn emit(self) {
        let event = ProfileEventLog {
            standard: "nep171-profile".to_string(),
            version: "1.0.0".to_string(),
            event: ProfileEventLogVariant::UnfollowProfile(self),
        };
        env::log_str(&event.to_string());
    }
}
