use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum VideoContext {
    NFT(VideoNFTId),
    Value(GovernedValueId),
    Proposal(ProposalId),
    // Choice(ChoiceId),
    // Calibration(CalibrationId),
    Rule(RuleId),
    Review(ReviewId, OriginalContext),
    // Violation(ViolationId, OriginalContext),
    // Possible types
    // Value(ValueId, VoteMethod),
    // Group(GroupId, VoteMethod),
}

impl Default for VideoContext {
    fn default() -> Self {
        VideoContext::NFT(0) // Replace with a sensible default TokenId
    }
}


#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum OriginalContext {
    NFT(TokenId),
    Value(GovernedValueId),
    Proposal(ProposalId),
    // Choice(ChoiceId),
    // Calibration(CalibrationId),
    Rule(RuleId),
}

impl From<VideoContext> for OriginalContext {
    fn from(context: VideoContext) -> Self {
        match context {
            VideoContext::NFT(token_id) => OriginalContext::NFT(token_id),
            VideoContext::Value(value_id) => OriginalContext::Value(value_id),
            VideoContext::Proposal(proposal_id) => OriginalContext::Proposal(proposal_id),
            // VideoContext::Choice(choice_id) => OriginalContext::Choice(choice_id),
            // VideoContext::Calibration(calibration_id) => OriginalContext::Calibration(calibration_id),
            VideoContext::Rule(rule_id) => OriginalContext::Rule(rule_id),
            VideoContext::Review(_, _) => panic!("Review context is not supported"),
        }
    }
}

impl From<OriginalContext> for VideoContext {
    fn from(context: OriginalContext) -> Self {
        match context {
            OriginalContext::NFT(token_id) => VideoContext::NFT(token_id),
            OriginalContext::Value(value_id) => VideoContext::Value(value_id),
            OriginalContext::Proposal(proposal_id) => VideoContext::Proposal(proposal_id),
            // OriginalContext::Choice(choice_id) => VideoContext::Choice(choice_id),
            // OriginalContext::Calibration(calibration_id) => VideoContext::Calibration(calibration_id),
            OriginalContext::Rule(rule_id) => VideoContext::Rule(rule_id),
        }
    }
}

