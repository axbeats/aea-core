use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ContractInput {
    pub contract_id: ContractId,
    pub dao_id: DaoId,
    // pub proposal_id: Option<ProposalId>,
    pub wasm_hash: CryptoHash,
    pub source_link: Option<String>,
    pub compiler_version: Option<String>,
    /// Video args
    pub name: String,
    pub description: Option<String>,
    pub video: VideoHash,
    pub image: ImageHash,
    pub location: Option<String>,
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
            wasm_hash: deploy_input.wasm_hash,
            source_link: deploy_input.source_code_link,
            compiler_version: deploy_input.compiler_version,
            name: deploy_input.name,
            description: deploy_input.description,
            video: deploy_input.video,
            image: deploy_input.image,
            location: deploy_input.location,
        }
    }

    // I THINK I NEED TO RE ARCHITECT THIS - Feb 26 2025
    // pub fn from_upgrade_contract_input(
    //     upgrade_input: UpgradeContractInput,
    //     video_id: VideoId,
    //     proposal_id: Option<ProposalId>,
    // ) -> Self {
    //     // Assume the current account is the DAO.
    //     let dao_id = env::current_account_id();
        
    //     Self {
    //         contract_id: upgrade_input.contract_id,
    //         dao_id,
    //         proposal_id,
    //         wasm_hash: upgrade_input.wasm_hash,
    //         source_link: upgrade_input.source_code_link,
    //         compiler_version: upgrade_input.compiler_version,
    //         name: upgrade_input.name,
    //         description: upgrade_input.description,
    //         video: upgrade_input.video,
    //         image: upgrade_input.image,
    //         location: upgrade_input.location,
    //     }
    // }

    pub fn from_add_managed_contract_input(
        register_input: AddManagedContractInput,
        // video_id: VideoId,
        // proposal_id: Option<ProposalId>,
    ) -> Self {
        let dao_id = env::current_account_id();
        
        Self {
            contract_id: register_input.contract_id,
            dao_id,
            // video_id,
            // proposal_id,
            wasm_hash: register_input.wasm_hash,
            // Map source_code_link to source_link.
            source_link: register_input.source_code_link,
            compiler_version: register_input.compiler_version,
            name: register_input.name,
            description: register_input.description,
            video: register_input.video,
            image: register_input.image,
            location: register_input.location,
        }
    }
}