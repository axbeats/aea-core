use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct VerifyValueResult {
    pub title: String,
    pub caption: Option<String>,
}
