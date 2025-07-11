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
