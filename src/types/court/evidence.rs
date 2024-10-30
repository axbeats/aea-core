use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum Evidence {
    Video(VideoHash),
    Image(ImageHash),
    Audio(AudioHash),
}