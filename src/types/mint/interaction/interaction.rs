use crate::*;

pub type MintWeight = i16;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum MintInteraction {
    Video(VideoMintInteraction),
    Vote(VoteMintInteraction),
    Court(CourtMintInteraction),
}
