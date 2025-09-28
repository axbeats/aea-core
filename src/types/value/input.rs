use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ValueInput {
    /// Value Args
    pub id: ValueId, // ContractId, FieldId, and Identifier - set by user
    pub dao_id: DaoId,
    pub operator_id: Option<ContractId>,
    pub method_input: VoteMethodInput,
    /// Video Args, note the target contract sets the name and description
    pub video_media: VideoMedia,
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
    pub video_media: Option<VideoMedia>,
}

impl ValueInput {
    pub fn from_video_option(
        input: ValueInputVideoOption,
        proposal_video: VideoMedia,
    ) -> Self {
        let video_media = input.video_media.unwrap_or(proposal_video);

        Self {
            id: input.id,
            dao_id: input.dao_id,
            operator_id: input.operator_id,
            method_input: input.method_input,
            video_media,
        }
    }

    pub fn unwrap_video_option(input: ValueInputVideoOption) -> Self {
        Self {
            id: input.id,
            dao_id: input.dao_id,
            operator_id: input.operator_id,
            method_input: input.method_input,
            video_media: input.video_media.unwrap(),
        }
    }
}