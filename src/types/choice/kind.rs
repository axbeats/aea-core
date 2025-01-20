use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub enum ChoiceKind {
    Single,
    Multiple(u8), // Number of elected items
}
