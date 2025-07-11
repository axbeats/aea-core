use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum UpdateRuleAction {
    Name(String),
    Description(String),
    VideoBundle(VideoBundle),
    FlagRole(RoleId),
    ReviewRole(RoleId),
    PenaltyFunction(FunctionCall),
    PenaltyAmount(Amount),
    Policy(RulePolicy),
}