use crate::*;

/// Filters for video search queries
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct VideoSearchFilters {
    /// Filter by video context types (e.g., Content, Proposal, Review)
    pub contexts: Option<Vec<VideoContextType>>,
    /// Filter by visibility (e.g., Public, Private, Subscription)
    pub visibility: Option<Vec<VideoVisibility>>,
    /// Filter by specific creator account IDs
    pub creator_ids: Option<Vec<AccountId>>,
    /// Minimum view count threshold
    pub min_views: Option<u64>,
    /// Minimum like count threshold
    pub min_likes: Option<u64>,
    /// Minimum comment count threshold
    pub min_comments: Option<u64>,
    /// Filter videos created after this timestamp (nanoseconds)
    pub created_after: Option<u64>,
    /// Filter videos created before this timestamp (nanoseconds)
    pub created_before: Option<u64>,
}

impl Default for VideoSearchFilters {
    fn default() -> Self {
        VideoSearchFilters {
            contexts: None,
            visibility: None,
            creator_ids: None,
            min_views: None,
            min_likes: None,
            min_comments: None,
            created_after: None,
            created_before: None,
        }
    }
}

/// Sorting strategies for video search results
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum VideoSortBy {
    /// Sort by text match score combined with engagement
    Relevance,
    /// Sort by creation date (newest first)
    MostRecent,
    /// Sort by total view count (highest first)
    MostViewed,
    /// Sort by total like count (highest first)
    MostLiked,
    /// Sort by total comment count (highest first)
    MostCommented,
    /// Sort by total favourite count (highest first)
    MostFavourited,
    /// Sort by trending score (recent engagement weighted)
    Trending,
}

impl Default for VideoSortBy {
    fn default() -> Self {
        VideoSortBy::MostRecent
    }
}
