use std::fmt;
use aea_macros::Generable;

use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq, Generable)]
pub enum ProfileKind {
    Personal,
    Business,
    Dao,
}

impl Default for ProfileKind {
    fn default() -> Self {
        ProfileKind::Personal
    }
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