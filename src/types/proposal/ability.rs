use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum ProposalAbility {
    Role,
    Policy,
    Task,
    Profile,
    Video,
    Code,
    Value,
    Court,
}