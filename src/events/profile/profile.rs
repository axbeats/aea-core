// use super::*;
// use crate::*;

// // Define the event variants for profile events
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(tag = "event", content = "data")]
// #[serde(rename_all = "snake_case")]
// #[serde(crate = "near_sdk::serde")]
// pub enum ProfileEventKind {
//     CreateProfile(CreateProfileEvent),
//     EditProfile(EditProfileEvent),
//     DeleteProfile(DeleteProfileEvent),
//     FollowProfile(FollowProfileEvent),
//     UnfollowProfile(UnfollowProfileEvent),
// }

// // Define the main ProfileEvent struct
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(crate = "near_sdk::serde")]
// pub struct ProfileEvent {
//     pub standard: String,
//     pub version: String,
//     #[serde(flatten)]
//     pub event: ProfileEventKind,
// }

// impl ProfileEvent {
//     pub fn new(event: ProfileEventKind) -> Self {
//         ProfileEvent {
//             standard: EVENT_STANDARD_NAME.to_string(),
//             version: EVENT_VERSION.to_string(),
//             event,
//         }
//     }
// }

// impl std::fmt::Display for ProfileEvent {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "EVENT_JSON:{}",
//             serde_json::to_string(self).map_err(|_| std::fmt::Error)?
//         )
//     }
// }

// Define separate structs for each action with necessary fields

// // CreateProfileEvent
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(crate = "near_sdk::serde")]
// pub struct CreateProfileEvent {
//     pub profile: Profile, // Assuming you have a Profile struct
// }

// impl CreateProfileEvent {
//     pub fn emit(self) {
//         let event = ProfileEvent::new(ProfileEventKind::CreateProfile(self));
//         env::log_str(&event.to_string());
//     }
// }

// impl EventKind for CreateProfileEvent {
//     fn event_kind(&self) -> &str {
//         "create_profile"
//     }
// }

// // EditProfileEvent
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(crate = "near_sdk::serde")]
// pub struct EditProfileEvent {
//     pub profile: Profile,
// }

// impl EditProfileEvent {
//     pub fn emit(self) {
//         let event = ProfileEvent::new(ProfileEventKind::EditProfile(self));
//         env::log_str(&event.to_string());
//     }
// }

// impl EventKind for EditProfileEvent {
//     fn event_kind(&self) -> &str {
//         "edit_profile"
//     }
// }

// // DeleteProfileEvent
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(crate = "near_sdk::serde")]
// pub struct DeleteProfileEvent {
//     pub account_id: AccountId,
//     pub timestamp: u64,
// }

// impl DeleteProfileEvent {
//     pub fn emit(self) {
//         let event = ProfileEvent::new(ProfileEventKind::DeleteProfile(self));
//         env::log_str(&event.to_string());
//     }
// }

// impl EventKind for DeleteProfileEvent {
//     fn event_kind(&self) -> &str {
//         "delete_profile"
//     }
// }

// // FollowProfileEvent
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(crate = "near_sdk::serde")]
// pub struct FollowProfileEvent {
//     pub follower_id: AccountId,
//     pub followed_id: AccountId,
//     pub timestamp: u64,
// }

// impl FollowProfileEvent {
//     pub fn emit(self) {
//         let event = ProfileEvent::new(ProfileEventKind::FollowProfile(self));
//         env::log_str(&event.to_string());
//     }
// }

// impl EventKind for FollowProfileEvent {
//     fn event_kind(&self) -> &str {
//         "follow_profile"
//     }
// }

// // UnfollowProfileEvent
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(crate = "near_sdk::serde")]
// pub struct UnfollowProfileEvent {
//     pub follower_id: AccountId,
//     pub followed_id: AccountId,
//     pub timestamp: u64,
// }

// impl UnfollowProfileEvent {
//     pub fn emit(self) {
//         let event = ProfileEvent::new(ProfileEventKind::UnfollowProfile(self));
//         env::log_str(&event.to_string());
//     }
// }

// impl EventKind for UnfollowProfileEvent {
//     fn event_kind(&self) -> &str {
//         "unfollow_profile"
//     }
// }
