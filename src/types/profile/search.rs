use crate::*;

/// Filters for profile/account search queries
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProfileSearchFilters {
    /// Filter by profile kind (Personal, Business, Dao)
    pub kinds: Option<Vec<ProfileKind>>,
    /// Filter by visibility (Public, Private)
    pub visibility: Option<Vec<ProfileVisibility>>,
    /// Minimum follower count threshold
    pub min_followers: Option<u64>,
    /// Minimum following count threshold
    pub min_following: Option<u64>,
    /// Filter profiles created after this timestamp (nanoseconds)
    pub joined_after: Option<u64>,
    /// Filter profiles created before this timestamp (nanoseconds)
    pub joined_before: Option<u64>,
}

impl Default for ProfileSearchFilters {
    fn default() -> Self {
        ProfileSearchFilters {
            kinds: None,
            visibility: None,
            min_followers: None,
            min_following: None,
            joined_after: None,
            joined_before: None,
        }
    }
}

/// Sorting strategies for profile search results
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum ProfileSortBy {
    /// Sort by text match score
    Relevance,
    /// Sort by join date (newest first)
    MostRecent,
    /// Sort by follower count (highest first)
    MostFollowers,
    /// Sort by following count (highest first)
    MostFollowing,
    /// Sort by trending score (engagement-based)
    Trending,
}

impl Default for ProfileSortBy {
    fn default() -> Self {
        ProfileSortBy::MostRecent
    }
}
