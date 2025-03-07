use std::fmt;

use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub enum ProfileKind {
    Personal,
    Business,
    Dao,
}

impl fmt::Display for ProfileKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProfileKind::Personal => write!(f, "personal"),
            ProfileKind::Business => write!(f, "business"),
            ProfileKind::Dao => write!(f, "dao"),
        }
    }
}