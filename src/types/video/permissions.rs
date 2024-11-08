// Import all items from the current crate.
use crate::*;

/// Represents the permissions associated with a video on the platform.
///
/// This struct contains various flags and settings that control who can interact with the video and how.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
// Specify that Serde should use the `near_sdk::serde` crate for serialization.
#[serde(crate = "near_sdk::serde")]
pub struct VideoPermissions {
    pub allow_comments: bool,                   // Flag indicating if comments are allowed on the video.
    pub allow_collaborations: bool,             // Flag indicating if collaborations are allowed on the video.
    pub allow_promoted: bool,                   // Flag indicating if the video can be promoted.
    pub approved_account_ids: HashMap<AccountId, u64>, // Mapping of approved account IDs to their corresponding approval IDs.
    pub next_approval_id: u64,                  // The next available approval ID.
}

impl Default for VideoPermissions {
    fn default() -> Self {
        VideoPermissions {
            allow_comments: false,
            allow_collaborations: false,
            allow_promoted: false,
            approved_account_ids: HashMap::new(),
            next_approval_id: 0,
        }
    }
}