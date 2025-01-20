use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum ReviewStatus {
    Open,
    Closed(ReviewVoteKind),
}
