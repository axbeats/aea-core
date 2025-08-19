use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Contract {
    pub id: ContractId,
    pub dao_id: DaoId,
    pub video_id: VideoId,
    pub repository_url: Option<String>,
    pub compiler_version: Option<String>,
    pub source_verified: Option<bool>,
    pub deployed_at: TimestampNanoSeconds,
}

impl Contract {
    pub fn from_input(input: ContractInput, video_id: VideoId) -> Self {
        Self {
            id: input.contract_id,
            dao_id: input.dao_id,
            video_id: video_id,
            repository_url: input.repository_url,
            compiler_version: input.compiler_version,
            source_verified: None,
            // Initially, there is no history of contract codes.
            deployed_at: env::block_timestamp(),
        }
    }

    pub fn upgrade_self(&mut self, input: UpgradeContractInput) -> Self {
        // Ensure that the provided identifiers match the existing contract.
        assert_eq!(self.id, input.contract_id, "ERR_CONTRACT_ID_MISMATCH");
        assert_eq!(self.dao_id, input.dao_id, "ERR_DAO_ID_MISMATCH");

        // Return the updated contract.
        Self {
            id: self.id.clone(),
            dao_id: self.dao_id.clone(),
            video_id: self.video_id.clone(),
            repository_url: input.repository_url,
            compiler_version: input.compiler_version,
            source_verified: None,
            deployed_at: env::block_timestamp(),
        }
    }
}