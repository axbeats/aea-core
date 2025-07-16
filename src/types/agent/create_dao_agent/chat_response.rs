use crate::*;
use near_sdk::serde_json;

/// UI element for embedding custom views in chat messages
#[near(serializers = [json, borsh])]
#[derive(Clone, Debug, Generable)]
pub struct UIElement {
    #[guide = "iOS view name to embed (e.g., 'CreateDaoNameView')"]
    pub view_name: String,
    
    #[guide = "Data binding path (e.g., 'profile.name')"]
    pub field_binding: Option<String>,
    
    #[guide = "Help text to display to the user"]
    pub help_text: Option<String>,
    
    #[guide = "Whether this field is required"]
    pub is_required: Option<bool>,
}

impl Default for UIElement {
    fn default() -> Self {
        Self {
            view_name: String::new(),
            field_binding: None,
            help_text: None,
            is_required: None,
        }
    }
}

impl UIElement {
    /// Convert to flat JSON format
    pub fn to_flat_json(&self) -> serde_json::Value {
        serde_json::json!({
            "view_name": self.view_name,
            "field_binding": self.field_binding,
            "help_text": self.help_text,
            "is_required": self.is_required,
        })
    }
    
    /// Parse from flat JSON format
    pub fn from_flat_json(value: &serde_json::Value) -> Result<Self, String> {
        let obj = value.as_object()
            .ok_or_else(|| "Expected JSON object for UIElement".to_string())?;
        
        let view_name = obj.get("view_name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Missing 'view_name' field".to_string())?
            .to_string();
        
        let field_binding = obj.get("field_binding")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
            
        let help_text = obj.get("help_text")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
            
        let is_required = obj.get("is_required")
            .and_then(|v| v.as_bool());
        
        Ok(UIElement {
            view_name,
            field_binding,
            help_text,
            is_required,
        })
    }
}

/// Response containing message, UI elements, and proposed actions
#[near(serializers = [json, borsh])]
#[derive(Clone, Debug, Generable)]
pub struct ChatResponse {
    #[guide = "The assistant's message"]
    pub message: String,
    
    #[guide = "UI elements to display"]
    pub ui_elements: Vec<UIElement>,
    
    #[guide = "Proposed actions for the user to approve"]
    pub proposed_actions: Vec<ProposedAction>,
    
    #[guide = "Current conversation state"]
    pub conversation_state: ConversationState,
}

impl Default for ChatResponse {
    fn default() -> Self {
        Self {
            message: "".to_string(),
            ui_elements: vec![],
            proposed_actions: vec![],
            conversation_state: ConversationState::GatheringIntent,
        }
    }
}

impl ChatResponse {
    /// Parse from a simplified flat JSON format that LLMs can generate more easily
    /// Example input:
    /// {
    ///   "message": "I'll help you create a token-based investment DAO...",
    ///   "conversation_state": "ProposingConfiguration",
    ///   "actions": [
    ///     {
    ///       "id": "action_001",
    ///       "action_type": "UpdateUsername",
    ///       "new_username": "my-dao",
    ///       "description": "Set the DAO username",
    ///       "rationale": "Clear identification",
    ///       "risk_level": "Low",
    ///       "affected_areas": ["Profile"]
    ///     }
    ///   ]
    /// }
    pub fn from_flat_json(value: &serde_json::Value) -> Result<Self, String> {
        let obj = value.as_object()
            .ok_or_else(|| "Expected JSON object".to_string())?;
        
        // Parse message
        let message = obj.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
            
        // Parse conversation state
        let conversation_state = match obj.get("conversation_state").and_then(|v| v.as_str()) {
            Some("GatheringIntent") => ConversationState::GatheringIntent,
            Some("ProposingConfiguration") => ConversationState::ProposingConfiguration,
            Some("WaitingForApproval") => ConversationState::WaitingForApproval,
            Some("ApplyingChanges") => ConversationState::ApplyingChanges,
            Some("Completed") => ConversationState::Completed,
            _ => ConversationState::GatheringIntent,
        };
        
        // Parse actions array
        let proposed_actions = if let Some(actions_arr) = obj.get("actions").and_then(|v| v.as_array()) {
            actions_arr.iter()
                .filter_map(|v| ProposedAction::from_flat_json(v).ok())
                .collect()
        } else {
            vec![]
        };
        
        // Parse UI elements array
        let ui_elements = if let Some(ui_arr) = obj.get("ui_elements").and_then(|v| v.as_array()) {
            ui_arr.iter()
                .filter_map(|v| UIElement::from_flat_json(v).ok())
                .collect()
        } else {
            vec![]
        };
        
        Ok(ChatResponse {
            message,
            ui_elements,
            proposed_actions,
            conversation_state,
        })
    }
    
    /// Convert to a simplified flat JSON format for LLM generation
    pub fn to_flat_json(&self) -> serde_json::Value {
        let flat_actions: Vec<serde_json::Value> = self.proposed_actions
            .iter()
            .map(|action| action.to_flat_json())
            .collect();
            
        let flat_ui_elements: Vec<serde_json::Value> = self.ui_elements
            .iter()
            .map(|ui| ui.to_flat_json())
            .collect();
            
        serde_json::json!({
            "message": self.message,
            "conversation_state": match self.conversation_state {
                ConversationState::GatheringIntent => "GatheringIntent",
                ConversationState::ProposingConfiguration => "ProposingConfiguration",
                ConversationState::WaitingForApproval => "WaitingForApproval",
                ConversationState::ApplyingChanges => "ApplyingChanges",
                ConversationState::Completed => "Completed",
            },
            "ui_elements": flat_ui_elements,
            "actions": flat_actions
        })
    }
}

/// State of the conversation
#[near(serializers = [json, borsh])]
#[derive(Clone, Debug, PartialEq, Generable)]
pub enum ConversationState {
    #[guide = "Gathering information about what the user wants"]
    GatheringIntent,
    
    #[guide = "Proposing a complete DAO configuration"]
    ProposingConfiguration,
    
    #[guide = "Waiting for user to approve/reject actions"]
    WaitingForApproval,
    
    #[guide = "Applying approved changes"]
    ApplyingChanges,
    
    #[guide = "DAO creation completed"]
    Completed,
}

impl Default for ConversationState {
    fn default() -> Self {
        Self::GatheringIntent
    }
}

/// Available iOS views for embedding
#[near(serializers = [json, borsh])]
#[derive(Clone, Debug, Generable)]
pub struct AvailableViews {
    #[guide = "Profile-related views"]
    pub profile: ProfileViews,
    
    #[guide = "Policy-related views"]
    pub policy: PolicyViews,
    
    #[guide = "Role-related views"]
    pub roles: RoleViews,
    
    #[guide = "Token-related views"]
    pub tokens: TokenViews,
    
    #[guide = "Rule-related views"]
    pub rules: RuleViews,
    
    #[guide = "Contract-related views"]
    pub contracts: ContractViews,
}

impl Default for AvailableViews {
    fn default() -> Self {
        Self {
            profile: ProfileViews::default(),
            policy: PolicyViews::default(),
            roles: RoleViews::default(),
            tokens: TokenViews::default(),
            rules: RuleViews::default(),
            contracts: ContractViews::default(),
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Clone, Debug, Generable)]
pub struct ProfileViews {
    pub username: String,
    pub name: String,
    pub bio: String,
    pub photo: String,
}

impl Default for ProfileViews {
    fn default() -> Self {
        Self {
            username: "CreateDaoUsernameView".to_string(),
            name: "CreateDaoNameView".to_string(),
            bio: "CreateDaoBioView".to_string(),
            photo: "CreateDaoPhotoView".to_string(),
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Clone, Debug, Generable)]
pub struct PolicyViews {
    pub threshold: String,
    pub quorum: String,
    pub voting_period: String,
    pub bond: String,
}

impl Default for PolicyViews {
    fn default() -> Self {
        Self {
            threshold: "CreateDaoThresholdComponent".to_string(),
            quorum: "CreateDaoQuorumComponent".to_string(),
            voting_period: "CreateDaoVotingPeriodComponent".to_string(),
            bond: "CreateDaoBondComponent".to_string(),
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Clone, Debug, Generable)]
pub struct RoleViews {
    pub name: String,
    pub description: String,
    pub kind: String,
    pub permissions: String,
    pub elected_members: String,
    pub video: String,
}

impl Default for RoleViews {
    fn default() -> Self {
        Self {
            name: "CreateDaoRoleNameComponent".to_string(),
            description: "CreateDaoRoleDescriptionComponent".to_string(),
            kind: "CreateDaoRoleKindComponent".to_string(),
            permissions: "CreateDaoRolePermissionsComponent".to_string(),
            elected_members: "CreateDaoElectedRoleView".to_string(),
            video: "CreateDaoRoleVideoComponent".to_string(),
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Clone, Debug, Generable)]
pub struct TokenViews {
    pub name: String,
    pub supply: String,
    pub price: String,
    pub sale_timeline: String,
}

impl Default for TokenViews {
    fn default() -> Self {
        Self {
            name: "CreateDaoTokenNameView".to_string(),
            supply: "CreateDaoTokenSupplyView".to_string(),
            price: "CreateDaoTokenPriceView".to_string(),
            sale_timeline: "CreateDaoTokenSaleTimelineView".to_string(),
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Clone, Debug, Generable)]
pub struct RuleViews {
    pub name: String,
    pub description: String,
    pub flag_role: String,
    pub flag_threshold: String,
    pub review_role: String,
    pub review_threshold: String,
    pub penalty_contract: String,
}

impl Default for RuleViews {
    fn default() -> Self {
        Self {
            name: "CreateDaoRuleNameView".to_string(),
            description: "CreateDaoRuleDescriptionView".to_string(),
            flag_role: "CreateDaoRuleFlagRoleView".to_string(),
            flag_threshold: "CreateDaoRuleFlagThresholdView".to_string(),
            review_role: "CreateDaoRuleReviewRoleView".to_string(),
            review_threshold: "CreateDaoRuleReviewThresholdView".to_string(),
            penalty_contract: "CreateDaoRulePenaltyContractIdView".to_string(),
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Clone, Debug, Generable)]
pub struct ContractViews {
    pub name: String,
    pub description: String,
    pub contract_id: String,
    pub source_link: String,
}

impl Default for ContractViews {
    fn default() -> Self {
        Self {
            name: "CreateDaoCodeNameView".to_string(),
            description: "CreateDaoCodeDescriptionView".to_string(),
            contract_id: "CreateDaoCodeContractIdView".to_string(),
            source_link: "CreateDaoCodeSourceLinkView".to_string(),
        }
    }
}