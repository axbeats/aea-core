use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ContractInput {
    pub contract_id: ContractId,
    pub dao_id: DaoId,
    pub video_id: VideoId,
    pub proposal_id: Option<ProposalId>,
    pub wasm_hash: CryptoHash,
    pub source_link: Option<String>,
    pub compiler_version: Option<String>,
}

impl ContractInput {
    pub fn from_deploy_contract_input(
        deploy_input: DeployContractInput,
        contract_id: ContractId,
        video_id: VideoId,
        proposal_id: Option<ProposalId>,
    ) -> Self {
        // In our design, the DAO ID is the current account ID.
        let dao_id = env::current_account_id();

        Self {
            contract_id,
            dao_id,
            video_id,
            proposal_id,
            wasm_hash: deploy_input.wasm_hash,
            source_link: deploy_input.source_code_link,
            compiler_version: deploy_input.compiler_version,
        }
    }

    pub fn from_upgrade_contract_input(
        upgrade_input: UpgradeContractInput,
        video_id: VideoId,
        proposal_id: Option<ProposalId>,
    ) -> Self {
        // Assume the current account is the DAO.
        let dao_id = env::current_account_id();
        
        Self {
            contract_id: upgrade_input.contract_id,
            dao_id,
            video_id,
            proposal_id,
            wasm_hash: upgrade_input.wasm_hash,
            // Map source_code_link to source_link.
            source_link: upgrade_input.source_code_link,
            compiler_version: upgrade_input.compiler_version,
        }
    }
}