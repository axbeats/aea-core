use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Contract {
    pub id: ContractId,
    pub dao_id: DaoId,
    pub video_id: VideoId,
    pub code: ContractCode,
    pub history: Option<Vec<ContractCode>>,
}

impl Contract {
    pub fn from_input(input: ContractInput, video_id: VideoId) -> Self {
        Self {
            id: input.contract_id,
            dao_id: input.dao_id,
            video_id: video_id,
            code: ContractCode {
                // proposal_id: input.proposal_id,
                // wasm_hash: input.wasm_hash,
                source_link: input.source_link,
                compiler_version: input.compiler_version,
                // Start with no verification; can be updated later.
                source_verified: None,
                // Use the current block timestamp as the deployment time.
                deployed_at: env::block_timestamp(),
            },
            // Initially, there is no history of contract codes.
            history: None,
        }
    }

    pub fn upgrade_self(&mut self, input: UpgradeContractInput) -> Self {
        // Ensure that the provided identifiers match the existing contract.
        assert_eq!(self.id, input.contract_id, "ERR_CONTRACT_ID_MISMATCH");
        assert_eq!(self.dao_id, input.dao_id, "ERR_DAO_ID_MISMATCH");
        // assert_eq!(self.video_id, input.video_id, "ERR_VIDEO_ID_MISMATCH");

        // Create new contract code from the input.
        let new_code = ContractCode {
            // proposal_id: input.proposal_id,
            // wasm_hash: input.wasm_hash,
            source_link: input.source_link,
            compiler_version: input.compiler_version,
            // New code starts with no verification.
            source_verified: None,
            // Set deployment time to the current block timestamp.
            deployed_at: env::block_timestamp(),
        };

        // Replace the current code with the new one,
        // capturing the old code.
        let old_code = std::mem::replace(&mut self.code, new_code);

        // Append the old code to the history.
        match &mut self.history {
            Some(history_vec) => history_vec.push(old_code),
            None => self.history = Some(vec![old_code]),
        }

        // Return the updated contract.
        Self {
            id: self.id.clone(),
            dao_id: self.dao_id.clone(),
            video_id: self.video_id.clone(),
            code: self.code.clone(),
            history: self.history.clone(),
        }
    }
}