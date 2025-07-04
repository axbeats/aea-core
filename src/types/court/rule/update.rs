use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum UpdateRuleAction {
    Name(String),
    Description(String),
    VideoBundle(VideoBundle),
    FlagGroup(GroupId),
    ReviewGroup(GroupId),
    PenaltyFunction(FunctionCall),
    PenaltyAmount(Amount),
    Policy(RulePolicy),
}