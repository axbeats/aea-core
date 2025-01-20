use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum GroupKind {
    // Anyone
    Elected(HashSet<AccountId>),
    Followers,
    // Point(Point),
    // Region(Region),.
    Subscribers,
    Token(StakingId),
}

impl GroupKind {
    /// Converts GroupKindInput to GroupKind using the provided StakingId
    pub fn from_input(input: GroupKindInput, staking_id: Option<StakingId>) -> Self {
        match input {
            GroupKindInput::Elected(members) => GroupKind::Elected(members.into_iter().collect()),
            GroupKindInput::Followers => GroupKind::Followers,
            GroupKindInput::Subscribers => GroupKind::Subscribers,
            // GroupKindInput::Token(_tuple) => GroupKind::Token(staking_id.unwrap()),
            GroupKindInput::Token => GroupKind::Token(staking_id.unwrap()),
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum GroupKindInput {
    Elected(Vec<AccountId>),
    Followers,
    // Point(Point),
    // Region(Region),
    Subscribers,
    // Token(TokenInitArgs),
    Token,
}