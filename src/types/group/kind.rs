use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum GroupKind {
    AI(GroupMembers),
    Elected(GroupMembers),
    Followers,
    // Point(Point),
    // Region(Region),
    Subscribers,
    Token(StakingId),
}

impl GroupKind {
    /// Converts GroupKindInput to GroupKind using the provided StakingId
    pub fn from_input(input: GroupKindInput, staking_id: Option<StakingId>) -> Self {
        match input {
            GroupKindInput::AI(local_group) => GroupKind::AI(local_group),
            GroupKindInput::Elected(local_group) => GroupKind::Elected(local_group),
            GroupKindInput::Followers => GroupKind::Followers,
            GroupKindInput::Subscribers => GroupKind::Subscribers,
            // GroupKindInput::Token(_tuple) => GroupKind::Token(staking_id.unwrap()),
            GroupKindInput::Token => GroupKind::Token(staking_id.unwrap()),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum GroupKindInput {
    AI(GroupMembers),
    Elected(GroupMembers),
    Followers,
    // Point(Point),
    // Region(Region),
    Subscribers,
    // Token(TokenInitArgs),
    Token,
}