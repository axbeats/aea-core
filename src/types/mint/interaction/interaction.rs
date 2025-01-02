use crate::*;

// pub type MintInteractionId = MethodName;
// pub type MintInteractionKey = (AccountId, MethodName);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[serde(crate = "near_sdk::serde")]
pub struct MintInteractionId {
    pub caller_id: AccountId,
    pub method_name: MethodName,
}

impl MintInteractionId {
    pub fn from_str(input: &str) -> Result<Self, &'static str> {
        // Split the input string into two parts (account_id and method_name)
        let parts: Vec<&str> = input.split(':').collect();
        if parts.len() != 2 {
            return Err("ERR_INVALID_FORMAT");
        }

        let account_id = parts[0].parse::<AccountId>().map_err(|_| "ERR_INVALID_ACCOUNT_ID")?;
        let method_name = parts[1].to_string();

        let key = MintInteractionId {
            caller_id: account_id,
            method_name: method_name,
        };

        Ok(key)
    }
}


pub type MintWeight = YoctoNumber;
pub type MethodName = String;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MintInteractions {
    pub interactions: UnorderedMap<MintInteractionId, MintInteraction>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MintInteraction {
    pub id: MintInteractionId,
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
