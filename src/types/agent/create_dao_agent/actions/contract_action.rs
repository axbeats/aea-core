use crate::*;

/// Actions that can be performed on DAO contracts
#[near(serializers = [json, borsh])]
#[serde(tag = "action", content = "params")]
#[derive(Debug, Clone, Generable)]
pub enum ContractAction {
    #[guide = "Register a new contract with the DAO"]
    AddContract {
        #[guide = "Complete contract configuration"]
        contract: ContractInput,
    },
    #[guide = "Remove a contract from the DAO"]
    RemoveContract {
        #[guide = "Zero-based index of the contract to remove"]
        contract_index: usize,
    },
    UpdateContractName { 
        contract_index: usize, 
        new_name: String 
    },
    UpdateContractDescription { 
        contract_index: usize, 
        new_description: String 
    },
    UpdateContractId { 
        contract_index: usize, 
        new_contract_id: String 
    },
    #[guide = "Set link to contract source code"]
    UpdateContractRepositoryUrl { 
        contract_index: usize, 
        #[guide = "URL to GitHub or other repository"]
        new_source_link: String 
    },
}

impl Default for ContractAction {
    fn default() -> Self {
        Self::AddContract { contract: ContractInput::default() }
    }
}