use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum VideoVisibility {
    Public,
    Private,
    Subscription,
}

impl Default for VideoVisibility {
    fn default() -> Self {
        VideoVisibility::Public
    }
}