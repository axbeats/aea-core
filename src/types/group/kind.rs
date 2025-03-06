use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum GroupKind {
    Anyone,
    Elected(ElectedGroup),
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
            GroupKindInput::Anyone => GroupKind::Anyone,
            GroupKindInput::Elected(group) => GroupKind::Elected(group),
            GroupKindInput::Followers => GroupKind::Followers,
            GroupKindInput::Subscribers => GroupKind::Subscribers,
            GroupKindInput::Token => GroupKind::Token(staking_id.unwrap()),
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum GroupKindInput {
    Anyone,
    Elected(ElectedGroup),
    Followers,
    // Point(Point),
    // Region(Region),
    Subscribers,
    // Token(TokenInitArgs),
    Token,
}