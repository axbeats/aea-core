use crate::*;

// // Input for a ProposalKind::CreateValue
// #[near(serializers = [json, borsh])]
// #[derive(Debug, Clone)]
// pub struct ValueInput {
//     pub id: ValueId, // ContractId, FieldId, and Identifier - set by user
//     pub operator_id: Option<ContractId>,
//     pub method_input: VoteMethodInput,
// }

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ValueInput {
    /// Value Args
    pub id: ValueId, // ContractId, FieldId, and Identifier - set by user
    pub dao_id: DaoId,
    pub operator_id: Option<ContractId>,
    pub method_input: VoteMethodInput,
    /// Video Args, note the target contract sets the name and description
    pub video: VideoHash,
    pub image: ImageHash,
    pub location: Option<String>,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ValueInputVideoOption {
    /// Value Args
    pub id: ValueId, // ContractId, FieldId, and Identifier - set by user
    pub dao_id: DaoId,
    pub operator_id: Option<ContractId>,
    pub method_input: VoteMethodInput,
    /// Video Args, note the target contract sets the name and description
    pub video_bundle: Option<VideoBundle>,
    pub location: Option<String>,
}

impl ValueInput {
    pub fn from_video_option(
        input: ValueInputVideoOption,
        proposal_video: VideoHash,
        proposal_image: ImageHash,
    ) -> Self {
        Self {
            id: input.id,
            dao_id: input.dao_id,
            operator_id: input.operator_id,
            method_input: input.method_input,
            video: proposal_video,
            image: proposal_image,
            location: input.location,
        }
    }

    pub fn unwrap_video_option(input: ValueInputVideoOption) -> Self {

        let bundle = input.video_bundle.unwrap();

        Self {
            id: input.id,
            dao_id: input.dao_id,
            operator_id: input.operator_id,
            method_input: input.method_input,
            video: bundle.video,
            image: bundle.image,
            location: input.location,
        }
    }
}