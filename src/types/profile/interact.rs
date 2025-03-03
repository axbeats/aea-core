use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum InteractProfile {
    Follow(AccountId),
    Unfollow(AccountId),
    Subscribe(AccountId),
    Unsubscribe(AccountId),
}