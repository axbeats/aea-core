use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProfileInteraction {
    pub following: bool,
    pub followed: bool,
    // pub subscribed: bool,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum InteractProfile {
    Follow(AccountId),
    Unfollow(AccountId),
    Subscribe(AccountId),
    Unsubscribe(AccountId),
}