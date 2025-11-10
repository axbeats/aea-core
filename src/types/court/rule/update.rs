use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum UpdateRuleAction {
    Name(String),
    Caption(Option<String>),
    VideoMedia(VideoMedia),
    FlagRole(RoleId),
    ReviewRole(RoleId),
    PenaltyFunction(FunctionCall),
    PenaltyAmount(Amount),
    Policy(RulePolicy),
}