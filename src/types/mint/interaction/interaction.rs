use crate::*;

// pub type MintInteractionId = MethodName;
pub type MintInteractionKey = (AccountId, MethodName);
pub type MintWeight = i16;
pub type MethodName = String;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MintInteractions {
    pub interactions: UnorderedMap<MintInteractionKey, MintInteraction>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MintInteraction {
    pub contract_id: AccountId,
    pub method_name: MethodName,
    pub stream_id: MintStreamId,
    pub weight: MintWeight,
}

// Users replace this with their own enum
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(crate = "near_sdk::serde")]
pub enum DefaultInteractions {
    // Video
    LikeVideo,
    UnlikeVideo,
    CommentVideo,
    DeleteCommentVideo,
    FavoriteVideo,
    UnfavoriteVideo,
    ShareVideo,
    UnshareVideo,
    ViewVideo,
    Reply,
    DeleteReply,
    LikeComment,
    UnlikeComment,
    LikeReply,
    UnlikeReply,
    // Vote
    CastProposalVote,
    CastChoiceVote,
    CastCalibrationVote,
    ReceiveApproveVote,
    ReceiveRejectVote,
    ReceiveSpamVote,
    // Court
    GuiltyAccusation,
    InnocentAccusation,
    CastReviewVote,
}
