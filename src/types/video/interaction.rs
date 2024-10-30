use crate::*;

/// `VideoInteraction` struct provides a concise summary of a user's interactions with a specific video.
/// This is particularly useful for displaying interaction data in a frontend application or for processing
/// within the contract.
///
/// Attributes:
/// - `viewed`: Indicates whether the user has viewed the video (`true` if viewed, `false` otherwise).
/// - `liked`: Indicates whether the user has liked the video (`true` if liked, `false` otherwise).
/// - `favourited`: Indicates whether the user has marked the video as a favourite (`true` if favourited, `false` otherwise).
/// - `commented`: Indicates whether the user has commented on the video (`true` if commented, `false` otherwise).
/// - `shared`: Indicates whether the user has shared the video (`true` if shared, `false` otherwise).
/// - `collaborated`: Indicates whether the user has collaborated on the video in some capacity, like as a co-creator or contributor (`true` if collaborated, `false` otherwise).
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VideoInteraction {
    pub viewed: bool,
    pub liked: bool,
    pub favourited: bool,
    pub commented: bool,
    pub shared: bool,
    pub collaborated: bool,
}