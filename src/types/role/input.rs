use crate::*;
use crate::types::config::FTSaleInput;
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
    pub caption: Option<String>,
    pub video_media: VideoMedia,
}

impl Default for RoleInput {
    fn default() -> Self {
        Self::example_followers()
    }
}

impl RoleInput {
    pub fn from_video_option(
        input: RoleInputVideoOption,
        proposal_video: VideoMedia,
    ) -> Self {
        let video_media = input.video_media.unwrap_or(proposal_video);

        Self {
            // Role fields
            dao_id: input.dao_id,
            kind: input.kind,
            permissions: input.permissions,
            // Video fields
            name: input.name,
            caption: input.description,
            video_media,
        }
    }

    pub fn unwrap_video_option(input: RoleInputVideoOption) -> Self {
        Self {
            // Role fields
            dao_id: input.dao_id,
            kind: input.kind,
            permissions: input.permissions,
            // Video fields
            name: input.name,
            caption: input.description,
            video_media: input.video_media.unwrap(),
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
            caption: Some("All followers of the DAO".to_string()),
            video_media: VideoMedia {
                hash: "QmExampleVideoHash".to_string(),
                ipfs_hash: Some("QmExampleIpfsHash".to_string()),
                streaming_url: "https://cloudflarestream.com/example-video-id/manifest/video.m3u8".to_string(),
                file_size: 1024000,
                duration: 60,
                resolution: VideoResolution { width: 1920, height: 1080 },
                thumbnail_timestamp: 1,
            },
        }
    }

    /// Generate an example Token role
    pub fn example_token() -> Self {
        Self {
            dao_id: "example-dao.near".parse().unwrap(),
            kind: RoleKindInput::Token(TokenRoleInput {
                weight_formula: WeightFormula::Linear,
                name: "Example Token".to_string(),
                symbol: "EXT".to_string(),
                icon: String::new(),
                total_supply: U128(1000000),
                decimals: 24,
                sale: FTSaleInput::default(),
            }),
            permissions: Self::example_permissions(),
            name: "Token Holders".to_string(),
            caption: Some("Token holders with staked tokens".to_string()),
            video_media: VideoMedia {
                hash: "QmExampleVideoHash".to_string(),
                ipfs_hash: Some("QmExampleIpfsHash".to_string()),
                streaming_url: "https://cloudflarestream.com/example-video-id/manifest/video.m3u8".to_string(),
                file_size: 1024000,
                duration: 60,
                resolution: VideoResolution { width: 1920, height: 1080 },
                thumbnail_timestamp: 1,
            },
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
            caption: Some("Elected council members".to_string()),
            video_media: VideoMedia {
                hash: "QmExampleVideoHash".to_string(),
                ipfs_hash: Some("QmExampleIpfsHash".to_string()),
                streaming_url: "https://cloudflarestream.com/example-video-id/manifest/video.m3u8".to_string(),
                file_size: 1024000,
                duration: 60,
                resolution: VideoResolution { width: 1920, height: 1080 },
                thumbnail_timestamp: 1,
            },
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
    pub video_media: Option<VideoMedia>,
}
