// Import all items from the current crate.
use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct VideoPermissions {
    pub allow_likes: bool,
    pub allow_comments: bool,
    pub allow_favourites: bool,
    pub allow_shares: bool,
    pub allow_collaborations: bool,
    pub allow_promoted: bool,
}

impl Default for VideoPermissions {
    fn default() -> Self {
        VideoPermissions {
            allow_likes: false,
            allow_comments: false,
            allow_favourites: false,
            allow_shares: false,
            allow_collaborations: false,
            allow_promoted: false,

        }
    }
}

impl VideoPermissions {
    pub fn vote_permissions() -> Self {
        VideoPermissions {
            allow_likes: false,
            allow_comments: true,
            allow_favourites: false,
            allow_shares: true,
            allow_collaborations: true,
            allow_promoted: false,
        }
    }

    pub fn group_permissions() -> Self {
        VideoPermissions {
            allow_likes: false,
            allow_comments: false,
            allow_favourites: false,
            allow_shares: true,
            allow_collaborations: false,
            allow_promoted: false,
        }
    }
}