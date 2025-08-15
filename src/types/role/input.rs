use crate::*;
use std::collections::{HashMap, HashSet};

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct RoleInput {
    /// Role fields
    pub dao_id: DaoId,
    pub kind: RoleKindInput,
    pub permissions: HashMap<ProposalAbility, ProposalPermission>,
    /// Video fields
    pub name: String,
    pub description: Option<String>,
    pub video: VideoHash,
    pub image: ImageHash,
    pub location: Option<String>,
}

impl Default for RoleInput {
    fn default() -> Self {
        Self::example_followers()
    }
}

impl RoleInput {
    pub fn from_video_option(
        input: RoleInputVideoOption,
        proposal_video: VideoHash,
        proposal_image: ImageHash,
    ) -> Self {
        let (video, image) = if let Some(bundle) = input.video_bundle {
            (bundle.video, bundle.image)
        } else {
            (proposal_video, proposal_image)
        };

        Self {
            // Role fields
            dao_id: input.dao_id,
            kind: input.kind,
            permissions: input.permissions,
            // Video fields
            name: input.name,
            description: input.description,
            video,
            image,
            location: input.location,
        }
    }

    pub fn unwrap_video_option(input: RoleInputVideoOption) -> Self {

        let bundle = input.video_bundle.unwrap();

        Self {
            // Role fields
            dao_id: input.dao_id,
            kind: input.kind,
            permissions: input.permissions,
            // Video fields
            name: input.name,
            description: input.description,
            video: bundle.video,
            image: bundle.image,
            location: input.location,
        }
    }

    /// Generate example permissions for a role
    fn example_permissions() -> HashMap<ProposalAbility, ProposalPermission> {
        let mut permissions = HashMap::new();
        permissions.insert(
            ProposalAbility::Role,
            ProposalPermission {
                create: true,
                vote: true,
            },
        );
        permissions.insert(
            ProposalAbility::Code,
            ProposalPermission {
                create: true,
                vote: true,
            },
        );
        permissions.insert(
            ProposalAbility::Task,
            ProposalPermission {
                create: true,
                vote: true,
            },
        );
        permissions
    }

    /// Generate an example Followers role
    pub fn example_followers() -> Self {
        Self {
            dao_id: "example-dao.near".parse().unwrap(),
            kind: RoleKindInput::Followers,
            permissions: Self::example_permissions(),
            name: "Followers".to_string(),
            description: Some("All followers of the DAO".to_string()),
            video: "QmExampleVideoHash".to_string(),
            image: "QmExampleImageHash".to_string(),
            location: None,
        }
    }

    /// Generate an example Token role
    pub fn example_token() -> Self {
        Self {
            dao_id: "example-dao.near".parse().unwrap(),
            kind: RoleKindInput::Token(TokenRoleInput {
                weight_formula: WeightFormula::Linear,
            }),
            permissions: Self::example_permissions(),
            name: "Token Holders".to_string(),
            description: Some("Token holders with staked tokens".to_string()),
            video: "QmExampleVideoHash".to_string(),
            image: "QmExampleImageHash".to_string(),
            location: None,
        }
    }

    /// Generate an example Elected role
    pub fn example_elected() -> Self {
        let mut members = HashSet::new();
        members.insert("alice.near".parse().unwrap());
        members.insert("bob.near".parse().unwrap());
        members.insert("charlie.near".parse().unwrap());
        
        Self {
            dao_id: "example-dao.near".parse().unwrap(),
            kind: RoleKindInput::Elected(ElectedRole {
                members,
                choice_id: 1, // Example choice ID
            }),
            permissions: Self::example_permissions(),
            name: "Council".to_string(),
            description: Some("Elected council members".to_string()),
            video: "QmExampleVideoHash".to_string(),
            image: "QmExampleImageHash".to_string(),
            location: None,
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct RoleInputVideoOption {
    /// Role fields
    pub dao_id: DaoId,
    pub kind: RoleKindInput,
    pub permissions: HashMap<ProposalAbility, ProposalPermission>,
    /// Video fields
    pub name: String,
    pub description: Option<String>,
    pub video_bundle: Option<VideoBundle>,
    pub location: Option<String>,
}
