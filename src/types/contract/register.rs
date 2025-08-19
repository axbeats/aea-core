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
    pub video: VideoHash,
    pub image: ImageHash,
    pub location: Option<String>,
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
    pub video_bundle: Option<VideoBundle>,
    pub location: Option<String>,
}

impl RegisterContractInput {
    pub fn from_video_option(
        input: RegisterContractInputVideoOption,
        proposal_video: VideoHash,
        proposal_image: ImageHash,
    ) -> Self {
        let (video, image) = if let Some(bundle) = input.video_bundle {
            (bundle.video, bundle.image)
        } else {
            (proposal_video, proposal_image)
        };

        Self {
            dao_id: input.dao_id,
            contract_id: input.contract_id,
            wasm_hash: input.wasm_hash,
            repository_url: input.repository_url,
            compiler_version: input.compiler_version,
            name: input.name,
            description: input.description,
            video,
            image,
            location: input.location,
        }
    }

    pub fn unwrap_video_option(input: RegisterContractInputVideoOption) -> Self {

        let bundle = input.video_bundle.unwrap();

        Self {
            dao_id: input.dao_id,
            contract_id: input.contract_id,
            wasm_hash: input.wasm_hash,
            repository_url: input.repository_url,
            compiler_version: input.compiler_version,
            name: input.name,
            description: input.description,
            video: bundle.video,
            image: bundle.image,
            location: input.location,
        }
    }
}