use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum GroupKind {
    Followers,
    Subscribers,
    Elected(ElectedGroup),    
    Token(StakingId),
    Region(RegionGroup),
}

impl GroupKind {
    /// Converts GroupKindInput to GroupKind using the provided StakingId
    pub fn from_input(input: GroupKindInput, staking_id: Option<StakingId>) -> Self {
        match input {
            GroupKindInput::Elected(group) => GroupKind::Elected(group),
            GroupKindInput::Followers => GroupKind::Followers,
            GroupKindInput::Subscribers => GroupKind::Subscribers,
            GroupKindInput::Token => GroupKind::Token(staking_id.unwrap()),
            GroupKindInput::Region(region) => GroupKind::Region(RegionGroup::from_input(region)),
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum GroupKindInput {
    Followers,
    Subscribers,
    Elected(ElectedGroup),
    Token,
    Region(RegionGroupInput),
}