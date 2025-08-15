use crate::*;

/// Comprehensive information about a user's roles and permissions for a specific DAO and ability
#[near(serializers = [json, borsh])]
#[derive(Clone, Debug)]
pub struct UserRoleInfo {
    /// Roles where we KNOW user is a member (Elected/Region/Agent)
    pub confirmed_member_roles: Vec<RoleWithPermission>,
    
    /// Roles that need external verification (Token/Follower/Subscriber)
    pub roles_needing_verification: Vec<RoleToVerify>,
    
    /// Pre-calculated: Can user create this type of proposal?
    pub can_create: bool,
    
    /// Pre-calculated: Can user vote on this type of proposal?
    pub can_vote: bool,
    
    /// All role IDs that can vote (for proposal initialization)
    pub voting_role_ids: Vec<RoleId>,
    
    /// If user can create, which role gives that permission
    pub creating_role_id: Option<RoleId>,
}

/// Information about a role where user membership is confirmed
#[near(serializers = [json, borsh])]
#[derive(Clone, Debug)]
pub struct RoleWithPermission {
    pub role_id: RoleId,
    pub role_name: String,
    pub can_create: bool,
    pub can_vote: bool,
    pub vote_weight_kind: VoteWeightKind,
}

/// Role that requires external contract verification
#[near(serializers = [json, borsh])]
#[derive(Clone, Debug)]
pub struct RoleToVerify {
    pub role_id: RoleId,
    pub role_name: String,
    pub verification_type: VerificationType,
    pub verification_target: AccountId, // StakingId or ProfileId
    pub permissions: ProposalPermission,
}

/// Type of external verification needed
#[near(serializers = [json, borsh])]
#[derive(Clone, Debug)]
pub enum VerificationType {
    Token(StakingId),
    Follower,
    Subscriber,
}

impl UserRoleInfo {
    /// Check if user needs any external verification
    pub fn needs_external_verification(&self) -> bool {
        !self.roles_needing_verification.is_empty()
    }
    
    /// Get all token verifications needed
    pub fn get_token_verifications(&self) -> Vec<(RoleId, StakingId)> {
        self.roles_needing_verification
            .iter()
            .filter_map(|role| {
                if let VerificationType::Token(staking_id) = &role.verification_type {
                    Some((role.role_id, staking_id.clone()))
                } else {
                    None
                }
            })
            .collect()
    }
    
    /// Check if follower verification is needed
    pub fn needs_follower_check(&self) -> bool {
        self.roles_needing_verification
            .iter()
            .any(|role| matches!(role.verification_type, VerificationType::Follower))
    }
    
    /// Check if subscriber verification is needed
    pub fn needs_subscriber_check(&self) -> bool {
        self.roles_needing_verification
            .iter()
            .any(|role| matches!(role.verification_type, VerificationType::Subscriber))
    }
}