use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum RoleKind {
    Followers,
    Subscribers,
    Elected(ElectedRole),    
    Token(StakingId),
    Region(RegionRole),
    Agent(AccountId),  // AI agent with its own NEAR account
}

impl RoleKind {
    /// Convert RoleKindInput to RoleKind using the provided StakingId
    pub fn from_input(input: RoleKindInput, staking_id: Option<StakingId>) -> Self {
        match input {
            RoleKindInput::Elected(role) => RoleKind::Elected(role),
            RoleKindInput::Followers => RoleKind::Followers,
            RoleKindInput::Subscribers => RoleKind::Subscribers,
            RoleKindInput::Token => RoleKind::Token(staking_id.unwrap()),
            RoleKindInput::Region(region) => RoleKind::Region(RegionRole::from_input(region)),
            RoleKindInput::Agent(account_id) => RoleKind::Agent(account_id),
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum RoleKindInput {
    Followers,
    Subscribers,
    Elected(ElectedRole),
    Token,
    Region(RegionRoleInput),
    Agent(AccountId),  // AI agent account ID
}