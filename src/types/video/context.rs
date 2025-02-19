use crate::*;

// THINK: Can I replace this with a Context Contract
// Allow users to build custom Video contexts
// ContextId -> ContractId w/ Id
// Feb 12 2025
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
#[serde(tag = "type")]
pub enum VideoContext {
    Nft { nft_id: VideoNftId },
    Value { value_id: ValueId },
    Proposal { proposal_id: ProposalId },
    Rule { rule_id: RuleId },
    Review { review_id: ReviewId, original: OriginalContext },
    // Product { product_id: ProductId },
    // Contract { contract_id: AccountId },
}

impl Default for VideoContext {
    fn default() -> Self {
        VideoContext::Nft {nft_id: 0 } // Replace with a sensible default TokenId
    }
}


#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum OriginalContext {
    Nft(TokenId),
    Value(ValueId),
    Proposal(ProposalId),
    Rule(RuleId),
}

impl From<VideoContext> for OriginalContext {
    fn from(context: VideoContext) -> Self {
        match context {
            VideoContext::Nft { nft_id } => OriginalContext::Nft(nft_id),
            VideoContext::Value { value_id } => OriginalContext::Value(value_id),
            VideoContext::Proposal { proposal_id } => OriginalContext::Proposal(proposal_id),
            VideoContext::Rule { rule_id } => OriginalContext::Rule(rule_id),
            VideoContext::Review { .. } => panic!("Review context is not supported"),
        }
    }
 }
 
 impl From<OriginalContext> for VideoContext {
    fn from(context: OriginalContext) -> Self {
        match context {
            OriginalContext::Nft(token_id) => VideoContext::Nft { nft_id: token_id },
            OriginalContext::Value(value_id) => VideoContext::Value { value_id },
            OriginalContext::Proposal(proposal_id) => VideoContext::Proposal { proposal_id },
            OriginalContext::Rule(rule_id) => VideoContext::Rule { rule_id },
        }
    }
 }
