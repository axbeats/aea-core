use crate::*;

// THINK: Can I replace this with a Context Contract
// Allow users to build custom Video contexts
// ContextId -> ContractId w/ Id
// Feb 12 2025
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
#[serde(tag = "type")]
pub enum VideoContext {
    Content { nft_id: VideoNftId },
    Proposal { proposal_id: ProposalId },
    Role { role_id: RoleId },
    Value { value_id: ValueId },
    Contract { contract_id: AccountId },
    Rule { rule_id: RuleId },
    Review { review_id: ReviewId, original: ReviewedContext },
    // Product { product_id: ProductId },
}

impl Default for VideoContext {
    fn default() -> Self {
        VideoContext::Content {nft_id: 0 } // Replace with a sensible default TokenId
    }
}

/// Simplified video context type for filtering (without associated data)
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VideoContextType {
    Content,
    Proposal,
    Role,
    Value,
    Contract,
    Rule,
    Review,
}

impl VideoContextType {
    /// Check if a VideoContext matches this type
    pub fn matches(&self, context: &VideoContext) -> bool {
        match (self, context) {
            (VideoContextType::Content, VideoContext::Content { .. }) => true,
            (VideoContextType::Proposal, VideoContext::Proposal { .. }) => true,
            (VideoContextType::Role, VideoContext::Role { .. }) => true,
            (VideoContextType::Value, VideoContext::Value { .. }) => true,
            (VideoContextType::Contract, VideoContext::Contract { .. }) => true,
            (VideoContextType::Rule, VideoContext::Rule { .. }) => true,
            (VideoContextType::Review, VideoContext::Review { .. }) => true,
            _ => false,
        }
    }

    /// Get the string representation for Neo4j queries
    pub fn to_query_string(&self) -> &str {
        match self {
            VideoContextType::Content => "Content",
            VideoContextType::Proposal => "Proposal",
            VideoContextType::Role => "Role",
            VideoContextType::Value => "Value",
            VideoContextType::Contract => "Contract",
            VideoContextType::Rule => "Rule",
            VideoContextType::Review => "Review",
        }
    }
}

impl From<&VideoContext> for VideoContextType {
    fn from(context: &VideoContext) -> Self {
        match context {
            VideoContext::Content { .. } => VideoContextType::Content,
            VideoContext::Proposal { .. } => VideoContextType::Proposal,
            VideoContext::Role { .. } => VideoContextType::Role,
            VideoContext::Value { .. } => VideoContextType::Value,
            VideoContext::Contract { .. } => VideoContextType::Contract,
            VideoContext::Rule { .. } => VideoContextType::Rule,
            VideoContext::Review { .. } => VideoContextType::Review,
        }
    }
}

// TODO: Build supporting Graph types - Oct 13 2025
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum VideoContextGraph {
    Content { nft_id: VideoNftId },
    Proposal { proposal_id: ProposalId },
    Role { role_id: RoleId },
    Value { value_id: ValueId },
    Contract { contract_id: AccountId },
    Rule { rule_id: RuleId },
    Review { review_id: ReviewId, original: ReviewedContext },
    // Product { product_id: ProductId },
    // Content { nft: VideoNftGraph },
    // Proposal { proposal: ProposalGraph },
    // Role { role: RoleGraph },
    // Value { value: ValueGraph },
    // Contract { contract: ContractGraph },
    // Rule { rule: RuleGraph },
    // Review {
    //     review: ReviewGraph,
    //     original: Box<VideoContextGraph>  // Recursive, needs Box
    // },
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum ReviewedContext {
    Nft(TokenId),
    Proposal(ProposalId),
    Value(ValueId),
    Role(RoleId),
    Contract(AccountId),
    Rule(RuleId),
}

impl From<VideoContext> for ReviewedContext {
    fn from(context: VideoContext) -> Self {
        match context {
            VideoContext::Content { nft_id } => ReviewedContext::Nft(nft_id),
            VideoContext::Proposal { proposal_id } => ReviewedContext::Proposal(proposal_id),
            VideoContext::Value { value_id } => ReviewedContext::Value(value_id),
            VideoContext::Role { role_id } => ReviewedContext::Role(role_id),
            VideoContext::Contract { contract_id } => ReviewedContext::Contract(contract_id),
            VideoContext::Rule { rule_id } => ReviewedContext::Rule(rule_id),
            VideoContext::Review { .. } => panic!("Review context is not supported"),
        }
    }
 }
 
 impl From<ReviewedContext> for VideoContext {
    fn from(context: ReviewedContext) -> Self {
        match context {
            ReviewedContext::Nft(token_id) => VideoContext::Content { nft_id: token_id },
            ReviewedContext::Proposal(proposal_id) => VideoContext::Proposal { proposal_id },
            ReviewedContext::Value(value_id) => VideoContext::Value { value_id },
            ReviewedContext::Role(role_id) => VideoContext::Role { role_id },
            ReviewedContext::Contract(contract_id) => VideoContext::Contract { contract_id },
            ReviewedContext::Rule(rule_id) => VideoContext::Rule { rule_id },
        }
    }
 }

// Temporary conversion from VideoContext to VideoContextGraph
// TODO: Replace this when proper context graph fetching is implemented
impl From<VideoContext> for VideoContextGraph {
    fn from(context: VideoContext) -> Self {
        match context {
            VideoContext::Content { nft_id } => VideoContextGraph::Content { nft_id },
            VideoContext::Proposal { proposal_id } => VideoContextGraph::Proposal { proposal_id },
            VideoContext::Role { role_id } => VideoContextGraph::Role { role_id },
            VideoContext::Value { value_id } => VideoContextGraph::Value { value_id },
            VideoContext::Contract { contract_id } => VideoContextGraph::Contract { contract_id },
            VideoContext::Rule { rule_id } => VideoContextGraph::Rule { rule_id },
            VideoContext::Review { review_id, original } => VideoContextGraph::Review { review_id, original },
        }
    }
}
