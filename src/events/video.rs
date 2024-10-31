use near_sdk::{env, serde::{Deserialize, Serialize}, serde_json};
use crate::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
pub enum VideoEventLogVariant {
    NftCreateVideo(NftCreateVideoLog),
    NftEditVideo(NftEditVideoLog),
    NftDeleteVideo(NftDeleteVideoLog),
    NftTransferVideo(NftTransferVideoLog),
    NftViewVideo(NftViewVideoLog),
    NftLikeVideo(NftLikeVideoLog),
    NftUnlikeVideo(NftUnlikeVideoLog),
    NftFavouriteVideo(NftFavouriteVideoLog),
    NftUnfavouriteVideo(NftUnfavouriteVideoLog),
    NftCommentVideo(NftCommentVideoLog),
    NftDeleteCommentVideo(NftDeleteCommentVideoLog),
    NftReplyCommentVideo(NftReplyCommentVideoLog),
    NftDeleteReplyCommentVideo(NftDeleteReplyCommentVideoLog),
    NftLikeCommentVideo(NftLikeCommentVideoLog),
    NftUnlikeCommentVideo(NftUnlikeCommentVideoLog),
    NftLikeReplyCommentVideo(NftLikeReplyCommentVideoLog),
    NftUnlikeReplyCommentVideo(NftUnlikeReplyCommentVideoLog),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct VideoEventLog {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: VideoEventLogVariant,
}

impl std::fmt::Display for VideoEventLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}

// Define separate structs for each action with necessary fields

// NftCreateVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftCreateVideoLog {
    pub video: Video,
}

impl NftCreateVideoLog {
    pub fn emit(self) {
        let event = VideoEventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: VideoEventLogVariant::NftCreateVideo(self),
        };
        env::log_str(&event.to_string());
    }
}

// NftEditVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftEditVideoLog {
    pub video: Video, // Not sure if I should emit entire video struct or an EditVideoEnum - Oct 30 2021
}

impl NftEditVideoLog {
    pub fn emit(self) {
        let event = VideoEventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: VideoEventLogVariant::NftEditVideo(self),
        };
        env::log_str(&event.to_string());
    }
}

// NftDeleteVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftDeleteVideoLog {
    pub owner_id: AccountId,
    pub video_id: VideoId,
    pub timestamp: u64,
}

impl NftDeleteVideoLog {
    pub fn emit(self) {
        let event = VideoEventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: VideoEventLogVariant::NftDeleteVideo(self),
        };
        env::log_str(&event.to_string());
    }
}

// NftTransferVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftTransferVideoLog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_id: Option<AccountId>,
    pub old_owner_id: AccountId,
    pub new_owner_id: AccountId,
    pub video_id: VideoId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    pub timestamp: u64,
}

impl NftTransferVideoLog {
    pub fn emit(self) {
        let event = VideoEventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: VideoEventLogVariant::NftTransferVideo(self),
        };
        env::log_str(&event.to_string());
    }
}

// NftViewVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftViewVideoLog {
    pub viewer_id: AccountId,
    pub video_id: VideoId,
    pub view_count: u64,
    pub view_length_in_ms: u64,
    pub timestamp: u64,
}

impl NftViewVideoLog {
    pub fn emit(self) {
        let event = VideoEventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: VideoEventLogVariant::NftViewVideo(self),
        };
        env::log_str(&event.to_string());
    }
}

// NftLikeVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftLikeVideoLog {
    pub liker_id: AccountId,
    pub video_id: VideoId,
    pub owner_id: AccountId,
    pub creator_id: AccountId,
    pub timestamp: u64,
}

impl NftLikeVideoLog {
    pub fn emit(self) {
        let event = VideoEventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: VideoEventLogVariant::NftLikeVideo(self),
        };
        env::log_str(&event.to_string());
    }
}

// NftUnlikeVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftUnlikeVideoLog {
    pub unliker_id: AccountId,
    pub video_id: VideoId,
    pub timestamp: u64,
}

impl NftUnlikeVideoLog {
    pub fn emit(self) {
        let event = VideoEventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: VideoEventLogVariant::NftUnlikeVideo(self),
        };
        env::log_str(&event.to_string());
    }
}

// NftFavouriteVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftFavouriteVideoLog {
    pub favouriter_id: AccountId,
    pub video_id: VideoId,
    pub owner_id: AccountId,
    pub creator_id: AccountId,
    pub timestamp: u64,
}

impl NftFavouriteVideoLog {
    pub fn emit(self) {
        let event = VideoEventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: VideoEventLogVariant::NftFavouriteVideo(self),
        };
        env::log_str(&event.to_string());
    }
}

// NftUnfavouriteVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftUnfavouriteVideoLog {
    pub unfavouriter_id: AccountId,
    pub video_id: VideoId,
    pub timestamp: u64,
}

impl NftUnfavouriteVideoLog {
    pub fn emit(self) {
        let event = VideoEventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: VideoEventLogVariant::NftUnfavouriteVideo(self),
        };
        env::log_str(&event.to_string());
    }
}

// NftCommentVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftCommentVideoLog {
    pub commenter_id: AccountId,
    pub video_id: VideoId,
    pub comment_id: CommentId,
    pub owner_id: AccountId,
    pub creator_id: AccountId,
    pub content: String,
    pub timestamp: u64,
}

impl NftCommentVideoLog {
    pub fn emit(self) {
        let event = VideoEventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: VideoEventLogVariant::NftCommentVideo(self),
        };
        env::log_str(&event.to_string());
    }
}

// NftDeleteCommentVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftDeleteCommentVideoLog {
    pub commenter_id: AccountId,
    pub video_id: VideoId,
    pub comment_id: CommentId,
    pub timestamp: u64,
}

impl NftDeleteCommentVideoLog {
    pub fn emit(self) {
        let event = VideoEventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: VideoEventLogVariant::NftDeleteCommentVideo(self),
        };
        env::log_str(&event.to_string());
    }
}

// NftReplyCommentVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftReplyCommentVideoLog {
    pub replier_id: AccountId,
    pub comment_id: CommentId,
    pub reply_id: ReplyId,
    pub owner_id: AccountId,
    pub creator_id: AccountId,
    pub content: String,
    pub timestamp: u64,
}

impl NftReplyCommentVideoLog {
    pub fn emit(self) {
        let event = VideoEventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: VideoEventLogVariant::NftReplyCommentVideo(self),
        };
        env::log_str(&event.to_string());
    }
}

// NftDeleteReplyCommentVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftDeleteReplyCommentVideoLog {
    pub replier_id: AccountId,
    pub comment_id: CommentId,
    pub reply_id: ReplyId,
    pub timestamp: u64,
}

impl NftDeleteReplyCommentVideoLog {
    pub fn emit(self) {
        let event = VideoEventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: VideoEventLogVariant::NftDeleteReplyCommentVideo(self),
        };
        env::log_str(&event.to_string());
    }
}

// NftLikeCommentVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftLikeCommentVideoLog {
    pub liker_id: AccountId,
    pub comment_id: CommentId,
    pub video_id: VideoId,
    pub owner_id: AccountId,
    pub creator_id: AccountId,
    pub timestamp: u64,
}

impl NftLikeCommentVideoLog {
    pub fn emit(self) {
        let event = VideoEventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: VideoEventLogVariant::NftLikeCommentVideo(self),
        };
        env::log_str(&event.to_string());
    }
}

// NftUnlikeCommentVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftUnlikeCommentVideoLog {
    pub unliker_id: AccountId,
    pub comment_id: CommentId,
    pub video_id: VideoId,
    pub owner_id: AccountId,
    pub creator_id: AccountId,
    pub timestamp: u64,
}

impl NftUnlikeCommentVideoLog {
    pub fn emit(self) {
        let event = VideoEventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: VideoEventLogVariant::NftUnlikeCommentVideo(self),
        };
        env::log_str(&event.to_string());
    }
}

// NftLikeReplyCommentVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftLikeReplyCommentVideoLog {
    pub liker_id: AccountId,
    pub reply_id: ReplyId,
    pub comment_id: CommentId,
    pub video_id: VideoId,
    pub owner_id: AccountId,
    pub creator_id: AccountId,
    pub timestamp: u64,
}

impl NftLikeReplyCommentVideoLog {
    pub fn emit(self) {
        let event = VideoEventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: VideoEventLogVariant::NftLikeReplyCommentVideo(self),
        };
        env::log_str(&event.to_string());
    }
}

// NftUnlikeReplyCommentVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftUnlikeReplyCommentVideoLog {
    pub unliker_id: AccountId,
    pub reply_id: ReplyId,
    pub comment_id: CommentId,
    pub video_id: VideoId,
    pub timestamp: u64,
}

impl NftUnlikeReplyCommentVideoLog {
    pub fn emit(self) {
        let event = VideoEventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: VideoEventLogVariant::NftUnlikeReplyCommentVideo(self),
        };
        env::log_str(&event.to_string());
    }
}
