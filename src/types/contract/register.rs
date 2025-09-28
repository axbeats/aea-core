use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct RegisterContractInput {
    pub dao_id: DaoId,
    pub contract_id: ContractId,
    pub wasm_hash: CryptoHash,
    pub repository_url: Option<String>,
    pub compiler_version: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub video_media: VideoMedia,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct RegisterContractInputVideoOption {
    pub dao_id: DaoId,
    pub contract_id: ContractId,
    pub wasm_hash: CryptoHash,
    pub repository_url: Option<String>,
    pub compiler_version: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub video_media: Option<VideoMedia>,
}

impl RegisterContractInput {
    pub fn from_video_option(
        input: RegisterContractInputVideoOption,
        proposal_video: VideoMedia,
    ) -> Self {
        let video = input.video_media.unwrap_or(proposal_video);

        Self {
            dao_id: input.dao_id,
            contract_id: input.contract_id,
            wasm_hash: input.wasm_hash,
            repository_url: input.repository_url,
            compiler_version: input.compiler_version,
            name: input.name,
            description: input.description,
            video_media: video,
        }
    }

    pub fn unwrap_video_option(input: RegisterContractInputVideoOption) -> Self {
        Self {
            dao_id: input.dao_id,
            contract_id: input.contract_id,
            wasm_hash: input.wasm_hash,
            repository_url: input.repository_url,
            compiler_version: input.compiler_version,
            name: input.name,
            description: input.description,
            video_media: input.video_media.unwrap(),
        }
    }
}