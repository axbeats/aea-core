use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Generable)]
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

impl Default for ProposalAbility {
    fn default() -> Self {
        Self::Role
    }
}