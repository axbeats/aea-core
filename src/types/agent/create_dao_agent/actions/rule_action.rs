use crate::*;

/// Actions that can be performed on DAO rules
#[near(serializers = [json, borsh])]
#[serde(tag = "action", content = "params")]
#[derive(Debug, Clone, Generable)]
pub enum RuleAction {
    #[guide = "Create a new community rule"]
    AddRule {
        #[guide = "Complete rule configuration"]
        rule: RuleInput,
    },
    #[guide = "Remove an existing rule"]
    RemoveRule {
        #[guide = "Index of the rule to remove"]
        rule_index: usize,
    },
    UpdateRuleName { 
        rule_index: usize, 
        new_name: String 
    },
    UpdateRuleDescription { 
        rule_index: usize, 
        new_description: String 
    },
    #[guide = "Set which role can flag violations of this rule"]
    UpdateRuleFlagRole { 
        rule_index: usize, 
        #[guide = "Index of the role that can flag violations"]
        role_index: usize 
    },
    #[guide = "Set threshold for flagging violations"]
    UpdateRuleFlagThreshold { 
        rule_index: usize, 
        #[guide = "Threshold in basis points (1000 = 10%)"]
        #[example = "1000"]
        new_threshold: u32
    },
    UpdateRuleFlagQuorum { 
        rule_index: usize, 
        new_quorum: u32 
    },
    #[guide = "Set which role reviews flagged violations"]
    UpdateRuleReviewRole { 
        rule_index: usize, 
        role_index: usize 
    },
    UpdateRuleReviewThreshold { 
        rule_index: usize, 
        new_threshold: u32
    },
    UpdateRuleReviewQuorum { 
        rule_index: usize, 
        new_quorum: u32 
    },
    #[guide = "Set contract that handles penalties"]
    UpdateRulePenaltyContractId { 
        rule_index: usize, 
        #[guide = "NEAR account ID of the penalty contract"]
        contract_id: String 
    },
    #[guide = "Set function to call for penalties"]
    UpdateRulePenaltyFunctionName { 
        rule_index: usize, 
        #[guide = "Name of the contract method to invoke"]
        function_name: String 
    },
}

impl Default for RuleAction {
    fn default() -> Self {
        Self::AddRule { rule: RuleInput::default() }
    }
}