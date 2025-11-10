use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct ContractInput {
    pub contract_id: ContractId,
    pub dao_id: DaoId,
    pub repository_url: Option<String>,
    pub compiler_version: Option<String>,
    /// Video args
    pub name: String,
    pub caption: Option<String>,
    pub video_media: VideoMedia,
}

impl Default for ContractInput {
    fn default() -> Self {
        Self::example()
    }
}

impl ContractInput {
    /// Generate an example ContractInput
    pub fn example() -> Self {
        Self {
            contract_id: "token.example-dao.near".parse().unwrap(),
            dao_id: "example-dao.near".parse().unwrap(),
            repository_url: Some("https://github.com/example-dao/token-contract".to_string()),
            compiler_version: Some("rustc 1.70.0".to_string()),
            name: "DAO Token Contract".to_string(),
            caption: Some("Fungible token contract for DAO governance".to_string()),
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

impl ContractInput {
    pub fn from_deploy_contract_input(
        deploy_input: DeployContractInput,
        contract_id: ContractId,
        // proposal_id: Option<ProposalId>,
    ) -> Self {
        // In our design, the DAO ID is the current account ID.
        let dao_id = env::current_account_id();

        Self {
            contract_id,
            dao_id,
            // proposal_id,
            // wasm_hash: deploy_input.wasm_hash,
            repository_url: deploy_input.repository_url,
            compiler_version: deploy_input.compiler_version,
            name: deploy_input.name,
            caption: deploy_input.description,
            video_media: deploy_input.video_media,
        }
    }

    pub fn from_add_managed_contract_input(
        register_input: RegisterContractInput,
        // video_id: VideoId,
        // proposal_id: Option<ProposalId>,
    ) -> Self {
        let dao_id = env::current_account_id();
        
        Self {
            contract_id: register_input.contract_id,
            dao_id,
            // video_id,
            // proposal_id,
            // wasm_hash: register_input.wasm_hash,
            // Map source_code_link to source_link.
            repository_url: register_input.repository_url,
            compiler_version: register_input.compiler_version,
            name: register_input.name,
            caption: register_input.description,
            video_media: register_input.video_media,
        }
    }
}