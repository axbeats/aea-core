use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct AddManagedContractInput {
    pub dao_id: DaoId,
    pub contract_id: ContractId,
    pub wasm_hash: CryptoHash,
    pub source_code_link: Option<String>,
    pub compiler_version: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub video: VideoHash,
    pub image: ImageHash,
    pub location: Option<String>,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct AddManagedContractInputVideoOption {
    pub dao_id: DaoId,
    pub contract_id: ContractId,
    pub wasm_hash: CryptoHash,
    pub source_code_link: Option<String>,
    pub compiler_version: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub video_bundle: Option<VideoBundle>,
    pub location: Option<String>,
}

impl AddManagedContractInput {
    pub fn from_video_option(
        input: AddManagedContractInputVideoOption,
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
            source_code_link: input.source_code_link,
            compiler_version: input.compiler_version,
            name: input.name,
            description: input.description,
            video,
            image,
            location: input.location,
        }
    }

    pub fn unwrap_video_option(input: AddManagedContractInputVideoOption) -> Self {

        let bundle = input.video_bundle.unwrap();

        Self {
            dao_id: input.dao_id,
            contract_id: input.contract_id,
            wasm_hash: input.wasm_hash,
            source_code_link: input.source_code_link,
            compiler_version: input.compiler_version,
            name: input.name,
            description: input.description,
            video: bundle.video,
            image: bundle.image,
            location: input.location,
        }
    }
}