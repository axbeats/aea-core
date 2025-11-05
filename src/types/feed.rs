use crate::*;

/// Simplified feed query - algorithm weights are fetched from the database
#[near(serializers = [json])]
#[derive(Debug, Clone)]
pub struct GetFeedQuery {
    pub account_id: String,
    pub limit: i64,
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
