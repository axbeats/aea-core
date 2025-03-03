use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum ContractUpdate {
    Name(String),
    Description(String),
    VideoBundle(VideoBundle),
}