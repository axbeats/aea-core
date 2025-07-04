use crate::*;

#[near(serializers = [json])]
#[derive(Debug, Clone)]
pub struct GetFeedQuery {
    pub account_id: String,
    pub limit: i64,

    pub entertainment_ratio: u8,
    pub governance_ratio: u8,
    pub court_ratio: u8,

    pub following_ratio: u8,
    pub recommended_ratio: u8,

    pub like_weight: u8,
    pub view_weight: u8,
    pub comment_weight: u8,

    pub interest_weight: u8,
    pub social_weight: u8,
    pub popularity_weight: u8,
    pub freshness_weight: u8,
    pub diversity_weight: u8,
    pub bridge_weight: u8,
}


// #[near(serializers = [json])]
// #[derive(Debug, Clone)]
// pub struct GetFeedQuery {
//     pub user_id: String,
//     pub limit: i64,

//     pub entertainment_ratio: f64,
//     pub governance_ratio: f64,
//     pub court_ratio: f64,

//     pub following_ratio: f64,
//     pub recommended_ratio: f64,

//     pub like_weight: f64,
//     pub view_weight: f64,
//     pub comment_weight: f64,

//     pub interest_weight: f64,
//     pub social_weight: f64,
//     pub popularity_weight: f64,
//     pub freshness_weight: f64,
//     pub diversity_weight: f64,
//     pub bridge_weight: f64,
// }
