use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum ProposalKind {
    Admin,
    Technical,
    Operations,
    Social,
}