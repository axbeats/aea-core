use crate::*;
use std::collections::{HashMap, HashSet};

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct GroupInput {
    /// Group fields
    pub dao_id: DaoId,
    pub kind: GroupKindInput,
    pub vote_weight: VoteWeightKind,
    pub permissions: HashMap<ProposalKind, ProposalPermission>,
    /// Video fields
    pub name: String,
    pub description: Option<String>,
    pub video: VideoHash,
    pub image: ImageHash,
    pub location: Option<String>,
}

impl GroupInput {
    pub fn from_video_option(
        input: GroupInputVideoOption,
        proposal_video: VideoHash,
        proposal_image: ImageHash,
    ) -> Self {
        let (video, image) = if let Some(bundle) = input.video_bundle {
            (bundle.video, bundle.image)
        } else {
            (proposal_video, proposal_image)
        };

        Self {
            // Group fields
            dao_id: input.dao_id,
            kind: input.kind,
            vote_weight: input.vote_weight,
            permissions: input.permissions,
            // Video fields
            name: input.name,
            description: input.description,
            video,
            image,
            location: input.location,
        }
    }

    pub fn unwrap_video_option(input: GroupInputVideoOption) -> Self {

        let bundle = input.video_bundle.unwrap();

        Self {
            // Group fields
            dao_id: input.dao_id,
            kind: input.kind,
            vote_weight: input.vote_weight,
            permissions: input.permissions,
            // Video fields
            name: input.name,
            description: input.description,
            video: bundle.video,
            image: bundle.image,
            location: input.location,
        }
    }

    /// Generate example permissions for a group
    fn example_permissions() -> HashMap<ProposalKind, ProposalPermission> {
        let mut permissions = HashMap::new();
        permissions.insert(
            ProposalKind::Admin,
            ProposalPermission {
                create: true,
                vote: true,
                custom_policy: None,
            },
        );
        permissions.insert(
            ProposalKind::Technical,
            ProposalPermission {
                create: true,
                vote: true,
                custom_policy: None,
            },
        );
        permissions.insert(
            ProposalKind::Operations,
            ProposalPermission {
                create: true,
                vote: true,
                custom_policy: None,
            },
        );
        permissions
    }

    /// Generate an example Followers group
    pub fn example_followers() -> Self {
        Self {
            dao_id: "example-dao.near".parse().unwrap(),
            kind: GroupKindInput::Followers,
            vote_weight: VoteWeightKind::Single,
            permissions: Self::example_permissions(),
            name: "Followers".to_string(),
            description: Some("All followers of the DAO".to_string()),
            video: "QmExampleVideoHash".to_string(),
            image: "QmExampleImageHash".to_string(),
            location: None,
        }
    }

    /// Generate an example Token group
    pub fn example_token() -> Self {
        Self {
            dao_id: "example-dao.near".parse().unwrap(),
            kind: GroupKindInput::Token,
            vote_weight: VoteWeightKind::Token((
                "staking.example-dao.near".parse().unwrap(),
                WeightFormula::Linear
            )),
            permissions: Self::example_permissions(),
            name: "Token Holders".to_string(),
            description: Some("Token holders with staked tokens".to_string()),
            video: "QmExampleVideoHash".to_string(),
            image: "QmExampleImageHash".to_string(),
            location: None,
        }
    }

    /// Generate an example Elected group
    pub fn example_elected() -> Self {
        let mut members = HashSet::new();
        members.insert("alice.near".parse().unwrap());
        members.insert("bob.near".parse().unwrap());
        members.insert("charlie.near".parse().unwrap());
        
        Self {
            dao_id: "example-dao.near".parse().unwrap(),
            kind: GroupKindInput::Elected(ElectedGroup {
                members,
                choice_id: 1, // Example choice ID
            }),
            vote_weight: VoteWeightKind::Single,
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
pub struct GroupInputVideoOption {
    /// Group fields
    pub dao_id: DaoId,
    pub kind: GroupKindInput,
    pub vote_weight: VoteWeightKind,
    pub permissions: HashMap<ProposalKind, ProposalPermission>,
    /// Video fields
    pub name: String,
    pub description: Option<String>,
    pub video_bundle: Option<VideoBundle>,
    pub location: Option<String>,
}
