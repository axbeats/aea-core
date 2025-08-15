use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum RoleKind {
    Followers,
    Subscribers,
    Elected(ElectedRole),    
    Token(TokenRole),
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
            RoleKindInput::Token(input) => RoleKind::Token(TokenRole {
                staking_id: staking_id.expect("ERR_TOKEN_ROLE_REQUIRES_STAKING_ID"),
                weight_formula: input.weight_formula,
            }),
            RoleKindInput::Region(region) => RoleKind::Region(RegionRole::from_input(region)),
            RoleKindInput::Agent(account_id) => RoleKind::Agent(account_id),
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub enum RoleKindInput {
    Followers,
    Subscribers,
    Elected(ElectedRole),
    Token(TokenRoleInput),
    Region(RegionRoleInput),
    Agent(AccountId),  // AI agent account ID
}

impl Default for RoleKindInput {
    fn default() -> Self {
        Self::Followers
    }
}

impl RoleKind {
    /// Get the staking contract ID if this is a token role
    pub fn get_staking_id(&self) -> Option<&StakingId> {
        match self {
            RoleKind::Token(token_role) => Some(&token_role.staking_id),
            _ => None,
        }
    }
}