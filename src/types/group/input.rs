use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct GroupInput {
    /// Group fields
    pub dao_id: DaoId,
    pub kind: GroupKindInput,
    pub vote_weight: VoteWeightKind,
    pub permissions: HashMap<ProposalKindString, ProposalPermission>,
    pub vote_method: VoteMethod,
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
            vote_method: input.vote_method,
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
            vote_method: input.vote_method,
            // Video fields
            name: input.name,
            description: input.description,
            video: bundle.video,
            image: bundle.image,
            location: input.location,
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
    pub permissions: HashMap<ProposalKindString, ProposalPermission>,
    pub vote_method: VoteMethod,
    /// Video fields
    pub name: String,
    pub description: Option<String>,
    pub video_bundle: Option<VideoBundle>,
    pub location: Option<String>,
}
