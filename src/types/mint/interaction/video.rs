use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum VideoMintInteraction {
    View(u8),
    ViewLengthInMS(TimestampMilliSeconds),
    Like,
    Unlike,
    Favourite,
    Unfavourite,
    Share,
    Comment,
    DeleteComment,
    Reply,
    DeleteReply,
    LikeComment,
    UnlikeComment,
    LikeReply,
    UnlikeReply,
}

impl VideoMintInteraction {
    pub fn total_weight(&self, weights: &VideoMintWeights) -> i64 {
        match self {
            VideoMintInteraction::View(view_count) => {
                *view_count as i64 * weights.view as i64
            }
            VideoMintInteraction::ViewLengthInMS(milliseconds) => {
                let seconds = (*milliseconds / 1000) as i64;
                seconds * weights.view_length_in_ms as i64
            }
            VideoMintInteraction::Like => weights.like as i64,
            VideoMintInteraction::Unlike => weights.unlike as i64,
            VideoMintInteraction::Favourite => weights.favourite as i64,
            VideoMintInteraction::Unfavourite => weights.unfavourite as i64,
            VideoMintInteraction::Share => weights.share as i64,
            VideoMintInteraction::Comment => weights.comment as i64,
            VideoMintInteraction::DeleteComment => weights.delete_comment as i64,
            VideoMintInteraction::Reply => weights.reply as i64,
            VideoMintInteraction::DeleteReply => weights.delete_reply as i64,
            VideoMintInteraction::LikeComment => weights.like_comment as i64,
            VideoMintInteraction::UnlikeComment => weights.unlike_comment as i64,
            VideoMintInteraction::LikeReply => weights.like_reply as i64,
            VideoMintInteraction::UnlikeReply => weights.unlike_reply as i64,
        }
    }
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VideoMintWeights {
    pub view: MintWeight,
    pub view_length_in_ms: MintWeight,
    pub like: MintWeight,
    pub unlike: MintWeight,
    pub favourite: MintWeight,
    pub unfavourite: MintWeight,
    pub share: MintWeight,
    pub comment: MintWeight,
    pub delete_comment: MintWeight,
    pub reply: MintWeight,
    pub delete_reply: MintWeight,
    pub like_comment: MintWeight,
    pub unlike_comment: MintWeight,
    pub like_reply: MintWeight,
    pub unlike_reply: MintWeight,
}
