use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProfileResults {
    pub profile_outputs: Vec<ProfileOutput>,
    pub count: u64,
    pub from_index: u64,
    pub limit: u64,
    pub has_more: bool,
}