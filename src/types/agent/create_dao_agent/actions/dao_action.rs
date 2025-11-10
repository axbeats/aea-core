use crate::*;
use near_sdk::serde_json;

/// Represents a specific action that can be performed on a DaoInput
#[near(serializers = [json, borsh])]
#[serde(tag = "type", content = "data")]
#[derive(Debug, Clone, Generable)]
pub enum DaoAction {
    Profile(ProfileAction),
    Role(RoleAction),
    Token(TokenAction),
    Policy(PolicyAction),
    Rule(RuleAction),
    Contract(ContractAction),
}

impl Default for DaoAction {
    fn default() -> Self {
        Self::Profile(ProfileAction::default())
    }
}

impl DaoAction {
    /// Parse from a simplified flat JSON format that LLMs can generate more easily
    /// Example input:
    /// {
    ///   "action_type": "UpdateUsername",
    ///   "new_username": "my-dao"
    /// }
    pub fn from_flat_json(value: &serde_json::Value) -> Result<Self, String> {
        let obj = value.as_object()
            .ok_or_else(|| "Expected JSON object".to_string())?;
        
        let action_type = obj.get("action_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Missing 'action_type' field".to_string())?;
        
        match action_type {
            // Profile actions
            "UpdateUsername" => {
                let new_username = obj.get("new_username")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'new_username' field".to_string())?
                    .to_string();
                Ok(DaoAction::Profile(ProfileAction::UpdateUsername { new_username }))
            },
            "UpdateName" => {
                let new_name = obj.get("new_name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'new_name' field".to_string())?
                    .to_string();
                Ok(DaoAction::Profile(ProfileAction::UpdateName { new_name }))
            },
            "UpdateBio" => {
                let new_bio = obj.get("new_bio")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                Ok(DaoAction::Profile(ProfileAction::UpdateBio { new_bio }))
            },
            
            // Role actions
            "AddRole" => {
                // For flat JSON, still accept the role as a nested object
                let role = obj.get("role")
                    .ok_or_else(|| "Missing 'role' field".to_string())?;
                let role_input: crate::RoleInput = serde_json::from_value(role.clone())
                    .map_err(|e| format!("Failed to parse RoleInput: {}", e))?;
                Ok(DaoAction::Role(RoleAction::AddRole { role: role_input }))
            },
            "RemoveRole" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                Ok(DaoAction::Role(RoleAction::RemoveRole { role_index }))
            },
            "UpdateRoleName" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let new_name = obj.get("new_name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'new_name' field".to_string())?
                    .to_string();
                Ok(DaoAction::Role(RoleAction::UpdateRoleName { role_index, new_name }))
            },
            "UpdateRoleDescription" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let new_description = obj.get("new_description")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'new_description' field".to_string())?
                    .to_string();
                Ok(DaoAction::Role(RoleAction::UpdateRoleDescription { role_index, new_description }))
            },
            "UpdateRoleKind" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let new_kind = obj.get("new_kind")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'new_kind' field".to_string())?
                    .to_string();
                Ok(DaoAction::Role(RoleAction::UpdateRoleKind { role_index, new_kind }))
            },
            "AddCreatePermissions" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let proposal_ability = obj.get("proposal_ability")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'proposal_ability' field".to_string())?
                    .to_string();
                Ok(DaoAction::Role(RoleAction::AddCreatePermissions { role_index, proposal_ability }))
            },
            "RemoveCreatePermissions" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let proposal_ability = obj.get("proposal_ability")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'proposal_ability' field".to_string())?
                    .to_string();
                Ok(DaoAction::Role(RoleAction::RemoveCreatePermissions { role_index, proposal_ability }))
            },
            "AddVotePermissions" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let proposal_ability = obj.get("proposal_ability")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'proposal_ability' field".to_string())?
                    .to_string();
                Ok(DaoAction::Role(RoleAction::AddVotePermissions { role_index, proposal_ability }))
            },
            "RemoveVotePermissions" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let proposal_ability = obj.get("proposal_ability")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'proposal_ability' field".to_string())?
                    .to_string();
                Ok(DaoAction::Role(RoleAction::RemoveVotePermissions { role_index, proposal_ability }))
            },
            "UpdateAgentAccount" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let agent_account_id = obj.get("agent_account_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'agent_account_id' field".to_string())?
                    .to_string();
                Ok(DaoAction::Role(RoleAction::UpdateAgentAccount { role_index, agent_account_id }))
            },
            "UpdateRegion" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let region_input = obj.get("region_input")
                    .ok_or_else(|| "Missing 'region_input' field".to_string())?;
                let region_input: crate::RegionRoleInput = serde_json::from_value(region_input.clone())
                    .map_err(|e| format!("Failed to parse region_input: {}", e))?;
                Ok(DaoAction::Role(RoleAction::UpdateRegion { role_index, region_input }))
            },
            "AddElectedMembers" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let members = obj.get("members")
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| "Missing 'members' field".to_string())?
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
                Ok(DaoAction::Role(RoleAction::AddElectedMembers { role_index, members }))
            },
            "RemoveElectedMembers" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let members = obj.get("members")
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| "Missing 'members' field".to_string())?
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
                Ok(DaoAction::Role(RoleAction::RemoveElectedMembers { role_index, members }))
            },
            
            // Token actions
            "UpdateTokenName" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let new_name = obj.get("new_name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'new_name' field".to_string())?
                    .to_string();
                Ok(DaoAction::Token(TokenAction::UpdateTokenName { role_index, new_name }))
            },
            "UpdateTokenSymbol" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let new_symbol = obj.get("new_symbol")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'new_symbol' field".to_string())?
                    .to_string();
                Ok(DaoAction::Token(TokenAction::UpdateTokenSymbol { role_index, new_symbol }))
            },
            "UpdateTotalSupply" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let new_supply = obj.get("new_supply")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'new_supply' field".to_string())?
                    .to_string();
                Ok(DaoAction::Token(TokenAction::UpdateTotalSupply { role_index, new_supply }))
            },
            "UpdateDecimals" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let new_decimals = obj.get("new_decimals")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'new_decimals' field".to_string())? as u8;
                Ok(DaoAction::Token(TokenAction::UpdateDecimals { role_index, new_decimals }))
            },
            "UpdateTokenSalePrice" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let new_price = obj.get("new_price")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'new_price' field".to_string())?
                    .to_string();
                Ok(DaoAction::Token(TokenAction::UpdateTokenSalePrice { role_index, new_price }))
            },
            "UpdateTokenSalePaymentCurrency" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let new_currency = obj.get("new_currency")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'new_currency' field".to_string())?
                    .to_string();
                Ok(DaoAction::Token(TokenAction::UpdateTokenSalePaymentCurrency { role_index, new_currency }))
            },
            "UpdateTokenSaleDeadline" => {
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                let deadline_days = obj.get("deadline_days")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'deadline_days' field".to_string())? as u32;
                let deadline_hours = obj.get("deadline_hours")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'deadline_hours' field".to_string())? as u32;
                let deadline_minutes = obj.get("deadline_minutes")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'deadline_minutes' field".to_string())? as u32;
                Ok(DaoAction::Token(TokenAction::UpdateTokenSaleDeadline { role_index, deadline_days, deadline_hours, deadline_minutes }))
            },
            
            // Policy actions
            "UpdateThreshold" => {
                let new_threshold = obj.get("new_threshold")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'new_threshold' field".to_string())? as u8;
                Ok(DaoAction::Policy(PolicyAction::UpdateThreshold { new_threshold }))
            },
            "UpdateEarlyThreshold" => {
                let new_early_threshold = obj.get("new_early_threshold")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as u8);
                Ok(DaoAction::Policy(PolicyAction::UpdateEarlyThreshold { new_early_threshold }))
            },
            "UpdateQuorum" => {
                let new_quorum = obj.get("new_quorum")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'new_quorum' field".to_string())? as u8;
                Ok(DaoAction::Policy(PolicyAction::UpdateQuorum { new_quorum }))
            },
            "UpdateEarlyQuorum" => {
                let new_early_quorum = obj.get("new_early_quorum")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as u8);
                Ok(DaoAction::Policy(PolicyAction::UpdateEarlyQuorum { new_early_quorum }))
            },
            "UpdateVotingPeriod" => {
                let days = obj.get("days")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'days' field".to_string())? as u32;
                let hours = obj.get("hours")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'hours' field".to_string())? as u32;
                let minutes = obj.get("minutes")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'minutes' field".to_string())? as u32;
                Ok(DaoAction::Policy(PolicyAction::UpdateVotingPeriod { days, hours, minutes }))
            },
            "UpdateBond" => {
                let new_bond = obj.get("new_bond")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'new_bond' field".to_string())?
                    .to_string();
                let currency = match obj.get("currency").and_then(|v| v.as_str()) {
                    Some("Near") => BondCurrency::Near,
                    Some("Aea") => BondCurrency::Aea,
                    Some(custom) => BondCurrency::Custom(custom.to_string()),
                    None => BondCurrency::Near,
                };
                Ok(DaoAction::Policy(PolicyAction::UpdateBond { new_bond, currency }))
            },
            
            // Rule actions
            "AddRule" => {
                let rule = obj.get("rule")
                    .ok_or_else(|| "Missing 'rule' field".to_string())?;
                let rule: RuleInput = serde_json::from_value(rule.clone())
                    .map_err(|e| format!("Failed to parse rule: {}", e))?;
                Ok(DaoAction::Rule(RuleAction::AddRule { rule }))
            },
            "UpdateRuleName" => {
                let rule_index = obj.get("rule_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'rule_index' field".to_string())? as usize;
                let new_name = obj.get("new_name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'new_name' field".to_string())?
                    .to_string();
                Ok(DaoAction::Rule(RuleAction::UpdateRuleName { rule_index, new_name }))
            },
            "UpdateRuleDescription" => {
                let rule_index = obj.get("rule_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'rule_index' field".to_string())? as usize;
                let new_description = obj.get("new_description")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'new_description' field".to_string())?
                    .to_string();
                Ok(DaoAction::Rule(RuleAction::UpdateRuleDescription { rule_index, new_description }))
            },
            "UpdateRuleFlagRole" => {
                let rule_index = obj.get("rule_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'rule_index' field".to_string())? as usize;
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                Ok(DaoAction::Rule(RuleAction::UpdateRuleFlagRole { rule_index, role_index }))
            },
            "UpdateRuleFlagThreshold" => {
                let rule_index = obj.get("rule_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'rule_index' field".to_string())? as usize;
                let new_threshold = obj.get("new_threshold")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'new_threshold' field".to_string())? as u32;
                Ok(DaoAction::Rule(RuleAction::UpdateRuleFlagThreshold { rule_index, new_threshold }))
            },
            "UpdateRuleFlagQuorum" => {
                let rule_index = obj.get("rule_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'rule_index' field".to_string())? as usize;
                let new_quorum = obj.get("new_quorum")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'new_quorum' field".to_string())? as u32;
                Ok(DaoAction::Rule(RuleAction::UpdateRuleFlagQuorum { rule_index, new_quorum }))
            },
            "UpdateRuleReviewRole" => {
                let rule_index = obj.get("rule_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'rule_index' field".to_string())? as usize;
                let role_index = obj.get("role_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'role_index' field".to_string())? as usize;
                Ok(DaoAction::Rule(RuleAction::UpdateRuleReviewRole { rule_index, role_index }))
            },
            "UpdateRuleReviewThreshold" => {
                let rule_index = obj.get("rule_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'rule_index' field".to_string())? as usize;
                let new_threshold = obj.get("new_threshold")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'new_threshold' field".to_string())? as u32;
                Ok(DaoAction::Rule(RuleAction::UpdateRuleReviewThreshold { rule_index, new_threshold }))
            },
            "UpdateRuleReviewQuorum" => {
                let rule_index = obj.get("rule_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'rule_index' field".to_string())? as usize;
                let new_quorum = obj.get("new_quorum")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'new_quorum' field".to_string())? as u32;
                Ok(DaoAction::Rule(RuleAction::UpdateRuleReviewQuorum { rule_index, new_quorum }))
            },
            "UpdateRulePenaltyContractId" => {
                let rule_index = obj.get("rule_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'rule_index' field".to_string())? as usize;
                let contract_id = obj.get("contract_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'contract_id' field".to_string())?
                    .to_string();
                Ok(DaoAction::Rule(RuleAction::UpdateRulePenaltyContractId { rule_index, contract_id }))
            },
            "UpdateRulePenaltyFunctionName" => {
                let rule_index = obj.get("rule_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'rule_index' field".to_string())? as usize;
                let function_name = obj.get("function_name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'function_name' field".to_string())?
                    .to_string();
                Ok(DaoAction::Rule(RuleAction::UpdateRulePenaltyFunctionName { rule_index, function_name }))
            },
            "RemoveRule" => {
                let rule_index = obj.get("rule_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'rule_index' field".to_string())? as usize;
                Ok(DaoAction::Rule(RuleAction::RemoveRule { rule_index }))
            },
            
            // Contract actions
            "AddContract" => {
                let contract = obj.get("contract")
                    .ok_or_else(|| "Missing 'contract' field".to_string())?;
                let contract: ContractInput = serde_json::from_value(contract.clone())
                    .map_err(|e| format!("Failed to parse contract: {}", e))?;
                Ok(DaoAction::Contract(ContractAction::AddContract { contract }))
            },
            "RemoveContract" => {
                let contract_index = obj.get("contract_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'contract_index' field".to_string())? as usize;
                Ok(DaoAction::Contract(ContractAction::RemoveContract { contract_index }))
            },
            "UpdateContractName" => {
                let contract_index = obj.get("contract_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing 'contract_index' field".to_string())? as usize;
                let new_name = obj.get("new_name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'new_name' field".to_string())?
                    .to_string();
                Ok(DaoAction::Contract(ContractAction::UpdateContractName { contract_index, new_name }))
            },
            
            // Add remaining actions as needed...
            _ => Err(format!("Unknown action type: {}", action_type))
        }
    }
    
    /// Convert to a simplified flat JSON format for LLM generation
    pub fn to_flat_json(&self) -> serde_json::Value {
        match self {
            DaoAction::Profile(action) => match action {
                ProfileAction::UpdateUsername { new_username } => {
                    serde_json::json!({
                        "action_type": "UpdateUsername",
                        "new_username": new_username
                    })
                },
                ProfileAction::UpdateName { new_name } => {
                    serde_json::json!({
                        "action_type": "UpdateName",
                        "new_name": new_name
                    })
                },
                ProfileAction::UpdateBio { new_bio } => {
                    serde_json::json!({
                        "action_type": "UpdateBio",
                        "new_bio": new_bio
                    })
                },
            },
            DaoAction::Role(action) => match action {
                RoleAction::AddRole { role } => {
                    serde_json::json!({
                        "action_type": "AddRole",
                        "role": role
                    })
                },
                RoleAction::RemoveRole { role_index } => {
                    serde_json::json!({
                        "action_type": "RemoveRole",
                        "role_index": role_index
                    })
                },
                RoleAction::UpdateRoleName { role_index, new_name } => {
                    serde_json::json!({
                        "action_type": "UpdateRoleName",
                        "role_index": role_index,
                        "new_name": new_name
                    })
                },
                // Add more role actions...
                _ => serde_json::json!({"action_type": "Unknown"})
            },
            // Add more categories...
            _ => serde_json::json!({"action_type": "Unknown"})
        }
    }
}

impl DaoAction {
    /// Get a human-readable description of what this action does
    pub fn get_description(&self) -> String {
        match self {
            DaoAction::Profile(action) => match action {
                ProfileAction::UpdateUsername { new_username } => {
                    format!("Update DAO username to '{}'", new_username)
                }
                ProfileAction::UpdateName { new_name } => {
                    format!("Update DAO name to '{}'", new_name)
                }
                ProfileAction::UpdateBio { new_bio } => {
                    match new_bio {
                        Some(bio) => format!("Update DAO bio to '{}'", bio),
                        None => "Remove DAO bio".to_string(),
                    }
                }
            },
            DaoAction::Role(action) => match action {
                RoleAction::AddRole { role } => {
                    format!("Create new role '{}'", role.name)
                }
                RoleAction::RemoveRole { role_index } => {
                    format!("Remove role at index {}", role_index)
                }
                RoleAction::UpdateRoleName { role_index, new_name } => {
                    format!("Update role {} name to '{}'", role_index, new_name)
                }
                RoleAction::UpdateRoleDescription { role_index, new_description } => {
                    format!("Update role {} description to '{}'", role_index, new_description)
                }
                RoleAction::UpdateRoleKind { role_index, new_kind } => {
                    format!("Update role {} type to {:?}", role_index, new_kind)
                }
                RoleAction::AddCreatePermissions { role_index, proposal_ability } => {
                    format!("Give role {} permission to create '{}' proposals", role_index, proposal_ability)
                }
                RoleAction::RemoveCreatePermissions { role_index, proposal_ability } => {
                    format!("Remove role {} permission to create '{}' proposals", role_index, proposal_ability)
                }
                RoleAction::AddVotePermissions { role_index, proposal_ability } => {
                    format!("Give role {} permission to vote on '{}' proposals", role_index, proposal_ability)
                }
                RoleAction::RemoveVotePermissions { role_index, proposal_ability } => {
                    format!("Remove role {} permission to vote on '{}' proposals", role_index, proposal_ability)
                }
                RoleAction::UpdateAgentAccount { role_index, agent_account_id } => {
                    format!("Update agent account for role {} to '{}'", role_index, agent_account_id)
                }
                RoleAction::UpdateRegion { role_index, region_input } => {
                    format!("Update region for role {} to '{}'", role_index, region_input.name)
                }
                RoleAction::AddElectedMembers { role_index, members } => {
                    format!("Add {} members to elected role {}", members.len(), role_index)
                }
                RoleAction::RemoveElectedMembers { role_index, members } => {
                    format!("Remove {} members from elected role {}", members.len(), role_index)
                }
            },
            DaoAction::Token(action) => match action {
                TokenAction::UpdateTokenName { role_index, new_name } => {
                    format!("Update token name for role {} to '{}'", role_index, new_name)
                }
                TokenAction::UpdateTokenSymbol { role_index, new_symbol } => {
                    format!("Update token symbol for role {} to '{}'", role_index, new_symbol)
                }
                TokenAction::UpdateTotalSupply { role_index, new_supply } => {
                    format!("Update total supply for role {} token to {:?}", role_index, new_supply)
                }
                TokenAction::UpdateDecimals { role_index, new_decimals } => {
                    format!("Update decimals for role {} token to {}", role_index, new_decimals)
                }
                TokenAction::UpdateTokenSalePrice { role_index, new_price } => {
                    format!("Update token sale price for role {} to {:?}", role_index, new_price)
                }
                TokenAction::UpdateTokenSalePaymentCurrency { role_index, new_currency } => {
                    format!("Update token sale payment currency for role {} to {}", role_index, new_currency)
                }
                TokenAction::UpdateTokenSaleDeadline { role_index, deadline_days, deadline_hours, deadline_minutes } => {
                    format!("Update token sale deadline for role {} to {} days, {} hours, {} minutes", 
                    role_index, deadline_days, deadline_hours, deadline_minutes)
                }
            },
            DaoAction::Policy(action) => match action {
                PolicyAction::UpdateThreshold { new_threshold } => {
                    format!("Update voting threshold to {}%", new_threshold)
                }
                PolicyAction::UpdateEarlyThreshold { new_early_threshold } => {
                    match new_early_threshold {
                        Some(threshold) => format!("Update early approval threshold to {}%", threshold),
                        None => "Disable early approval threshold".to_string(),
                    }
                }
                PolicyAction::UpdateQuorum { new_quorum } => {
                    format!("Update voting quorum to {}%", new_quorum)
                }
                PolicyAction::UpdateEarlyQuorum { new_early_quorum } => {
                    match new_early_quorum {
                        Some(quorum) => format!("Update early approval quorum to {}%", quorum),
                        None => "Disable early approval quorum".to_string(),
                    }
                }
                PolicyAction::UpdateVotingPeriod { days, hours, minutes } => {
                    format!("Update voting period to {} days, {} hours, {} minutes", days, hours, minutes)
                }
                PolicyAction::UpdateBond { new_bond, currency } => {
                    format!("Update proposal bond to {:?} {:?}", new_bond, currency)
                }
            },
            DaoAction::Rule(action) => match action {
                RuleAction::AddRule { rule } => {
                    format!("Add new rule '{}': {}", rule.name, rule.caption.as_deref().unwrap_or(""))
                }
                RuleAction::UpdateRuleName { rule_index, new_name } => {
                    format!("Update rule {} name to '{}'", rule_index, new_name)
                }
                RuleAction::UpdateRuleDescription { rule_index, new_description } => {
                    format!("Update rule {} description to '{}'", rule_index, new_description)
                }
                RuleAction::UpdateRuleFlagRole { rule_index, role_index } => {
                    format!("Set role {} as flag role for rule {}", role_index, rule_index)
                }
                RuleAction::UpdateRuleFlagThreshold { rule_index, new_threshold } => {
                    format!("Update rule {} flag threshold to {}%", rule_index, *new_threshold as f32 / 100.0)
                }
                RuleAction::UpdateRuleFlagQuorum { rule_index, new_quorum } => {
                    format!("Update rule {} flag quorum to {}", rule_index, new_quorum)
                }
                RuleAction::UpdateRuleReviewRole { rule_index, role_index } => {
                    format!("Set role {} as review role for rule {}", role_index, rule_index)
                }
                RuleAction::UpdateRuleReviewThreshold { rule_index, new_threshold } => {
                    format!("Update rule {} review threshold to {}%", rule_index, *new_threshold as f32 / 100.0)
                }
                RuleAction::UpdateRuleReviewQuorum { rule_index, new_quorum } => {
                    format!("Update rule {} review quorum to {}", rule_index, new_quorum)
                }
                RuleAction::UpdateRulePenaltyContractId { rule_index, contract_id } => {
                    format!("Set penalty contract for rule {} to '{}'", rule_index, contract_id)
                }
                RuleAction::UpdateRulePenaltyFunctionName { rule_index, function_name } => {
                    format!("Set penalty function for rule {} to '{}'", rule_index, function_name)
                }
                RuleAction::RemoveRule { rule_index } => {
                    format!("Remove rule at index {}", rule_index)
                }
            },
            DaoAction::Contract(action) => match action {
                ContractAction::AddContract { contract } => {
                    format!("Add new contract '{}' ({}): {}", 
                        contract.name, 
                        contract.contract_id, 
                        contract.caption.as_deref().unwrap_or(""))
                }
                ContractAction::RemoveContract { contract_index } => {
                    format!("Remove contract at index {}", contract_index)
                }
                ContractAction::UpdateContractName { contract_index, new_name } => {
                    format!("Update contract {} name to '{}'", contract_index, new_name)
                }
                ContractAction::UpdateContractDescription { contract_index, new_description } => {
                    format!("Update contract {} description to '{}'", contract_index, new_description)
                }
                ContractAction::UpdateContractId { contract_index, new_contract_id } => {
                    format!("Update contract {} ID to '{}'", contract_index, new_contract_id)
                }
                ContractAction::UpdateContractRepositoryUrl { contract_index, new_source_link } => {
                    format!("Update contract {} repository URL to '{}'", contract_index, new_source_link)
                }
            }
        }
    }

    /// Get the risk level of this action
    pub fn get_risk_level(&self) -> RiskLevel {
        match self {
            DaoAction::Profile(ProfileAction::UpdateName { .. }) |
            DaoAction::Profile(ProfileAction::UpdateBio { .. }) |
            DaoAction::Role(RoleAction::UpdateRoleName { .. }) |
            DaoAction::Role(RoleAction::UpdateRoleDescription { .. }) |
            DaoAction::Token(TokenAction::UpdateTokenName { .. }) |
            DaoAction::Token(TokenAction::UpdateTokenSymbol { .. }) |
            DaoAction::Rule(RuleAction::UpdateRuleName { .. }) |
            DaoAction::Rule(RuleAction::UpdateRuleDescription { .. }) |
            DaoAction::Contract(ContractAction::UpdateContractName { .. }) |
            DaoAction::Contract(ContractAction::UpdateContractDescription { .. }) => RiskLevel::Low,

            DaoAction::Policy(_) |
            DaoAction::Role(RoleAction::UpdateRoleKind { .. }) |
            DaoAction::Role(RoleAction::AddVotePermissions { .. }) |
            DaoAction::Role(RoleAction::RemoveVotePermissions { .. }) |
            DaoAction::Rule(RuleAction::UpdateRuleFlagRole { .. }) |
            DaoAction::Rule(RuleAction::UpdateRuleReviewRole { .. }) |
            DaoAction::Rule(RuleAction::UpdateRuleFlagThreshold { .. }) |
            DaoAction::Rule(RuleAction::UpdateRuleFlagQuorum { .. }) |
            DaoAction::Rule(RuleAction::UpdateRuleReviewThreshold { .. }) |
            DaoAction::Rule(RuleAction::UpdateRuleReviewQuorum { .. }) |
            DaoAction::Contract(ContractAction::UpdateContractId { .. }) |
            DaoAction::Contract(ContractAction::UpdateContractRepositoryUrl { .. }) => RiskLevel::Medium,

            DaoAction::Profile(ProfileAction::UpdateUsername { .. }) |
            DaoAction::Token(TokenAction::UpdateTotalSupply { .. }) |
            DaoAction::Token(TokenAction::UpdateDecimals { .. }) |
            DaoAction::Role(RoleAction::UpdateAgentAccount { .. }) |
            DaoAction::Role(RoleAction::UpdateRegion { .. }) |
            DaoAction::Rule(RuleAction::AddRule { .. }) |
            DaoAction::Rule(RuleAction::UpdateRulePenaltyContractId { .. }) |
            DaoAction::Rule(RuleAction::UpdateRulePenaltyFunctionName { .. }) |
            DaoAction::Contract(ContractAction::AddContract { .. }) => RiskLevel::High,

            _ => RiskLevel::Medium,
        }
    }
}

/// Helper enum for bond currency
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub enum BondCurrency {
    #[guide = "NEAR Protocol native token"]
    Near,
    #[guide = "AEA platform token"]
    Aea,
    #[guide = "Custom token by contract ID"]
    Custom(String),
}

impl Default for BondCurrency {
    fn default() -> Self {
        Self::Near
    }
}

/// A proposed action with metadata for user approval
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct ProposedAction {
    #[guide = "Unique identifier for this action"]
    pub id: String,
    #[guide = "The action to perform"]
    pub action: DaoAction,
    #[guide = "Human-readable description of what this does"]
    pub description: String,
    #[guide = "Explanation of why this action is suggested"]
    pub rationale: String,
    #[guide = "Potential impact and risks"]
    pub estimated_impact: ActionImpact,
    #[guide = "Current status of the action"]
    #[serde(default = "default_action_status")]
    pub status: ActionStatus,
}

impl Default for ProposedAction {
    fn default() -> Self {
        Self { 
            id: "".to_string(), 
            action: DaoAction::default(), 
            description: "".to_string(), 
            rationale: "".to_string(), 
            estimated_impact: ActionImpact::default(), 
            status: ActionStatus::Pending 
        }
    }
}

impl ProposedAction {
    /// Parse from a simplified flat JSON format that LLMs can generate more easily
    /// Example input:
    /// {
    ///   "id": "action_001",
    ///   "action_type": "UpdateUsername",
    ///   "new_username": "my-dao",
    ///   "description": "Set the DAO username",
    ///   "rationale": "Clear identification for your DAO",
    ///   "risk_level": "Low",
    ///   "affected_areas": ["Profile"],
    ///   "dependencies": []
    /// }
    pub fn from_flat_json(value: &serde_json::Value) -> Result<Self, String> {
        let obj = value.as_object()
            .ok_or_else(|| "Expected JSON object".to_string())?;
        
        // Parse the action using DaoAction's flat format
        let action = DaoAction::from_flat_json(value)?;
        
        // Parse other fields
        let id = obj.get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("generated_id")
            .to_string();
            
        let description = obj.get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
            
        let rationale = obj.get("rationale")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
            
        // Parse risk level
        let risk_level = match obj.get("risk_level").and_then(|v| v.as_str()) {
            Some("Low") => RiskLevel::Low,
            Some("Medium") => RiskLevel::Medium,
            Some("High") => RiskLevel::High,
            _ => RiskLevel::Low,
        };
        
        // Parse affected areas
        let affected_areas = obj.get("affected_areas")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect())
            .unwrap_or_default();
            
        // Parse dependencies
        let dependencies = obj.get("dependencies")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect())
            .unwrap_or_default();
            
        Ok(ProposedAction {
            id,
            action,
            description,
            rationale,
            estimated_impact: ActionImpact {
                risk_level,
                affected_areas,
                dependencies,
            },
            status: ActionStatus::Pending,
        })
    }
    
    /// Convert to a simplified flat JSON format for LLM generation
    pub fn to_flat_json(&self) -> serde_json::Value {
        let mut flat_action = self.action.to_flat_json();
        
        // Add ProposedAction fields to the flat representation
        if let Some(obj) = flat_action.as_object_mut() {
            obj.insert("id".to_string(), serde_json::json!(self.id.clone()));
            obj.insert("description".to_string(), serde_json::json!(self.description.clone()));
            obj.insert("rationale".to_string(), serde_json::json!(self.rationale.clone()));
            obj.insert("risk_level".to_string(), serde_json::json!(
                match self.estimated_impact.risk_level {
                    RiskLevel::Low => "Low",
                    RiskLevel::Medium => "Medium",
                    RiskLevel::High => "High",
                }.to_string()
            ));
            obj.insert("affected_areas".to_string(), serde_json::json!(self.estimated_impact.affected_areas));
            obj.insert("dependencies".to_string(), serde_json::json!(self.estimated_impact.dependencies));
        }
        
        flat_action
    }
}

/// Default status for new actions
fn default_action_status() -> ActionStatus {
    ActionStatus::Pending
}

/// Describes the impact of an action
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct ActionImpact {
    #[guide = "Risk level of this change"]
    pub risk_level: RiskLevel,
    #[guide = "Which parts of the DAO are affected"]
    pub affected_areas: Vec<String>,
    #[guide = "Other actions that must be completed first"]
    pub dependencies: Vec<String>,
}

impl Default for ActionImpact {
    fn default() -> Self {
        Self { risk_level: RiskLevel::Low, affected_areas: vec![], dependencies: vec![] }
    }
}

/// Risk level for actions
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq, Generable)]
pub enum RiskLevel {
    #[guide = "Simple updates like name changes"]
    Low,
    #[guide = "Policy changes that affect governance"]
    Medium,
    #[guide = "Major structural changes"]
    High,
}

impl Default for RiskLevel {
    fn default() -> Self {
        Self::Low
    }
}

/// Status of actions
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq, Generable)]
pub enum ActionStatus {
    #[guide = "Waiting for user approval"]
    Pending,
    #[guide = "User has approved this action"]
    Approved,
    #[guide = "User has rejected this action"]
    Rejected,
}

impl Default for ActionStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Response from Claude containing actions
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct ActionBasedResponse {
    #[guide = "Natural language message from the assistant"]
    pub message: String,
    #[guide = "List of suggested actions"]
    pub proposed_actions: Vec<ProposedAction>,
    #[guide = "Context about the conversation state"]
    pub conversation_context: String,
}

impl Default for ActionBasedResponse {
    fn default() -> Self {
        Self { message: "".to_string(), proposed_actions: vec![], conversation_context: "".to_string() }
    }
}