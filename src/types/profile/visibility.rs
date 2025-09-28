use crate::*;
use std::fmt;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub enum ProfileVisibility {
    Public,
    Private,
}

impl Default for ProfileVisibility {
    fn default() -> Self {
        ProfileVisibility::Public
    }
}

impl fmt::Display for ProfileVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProfileVisibility::Public => write!(f, "Public"),
            ProfileVisibility::Private => write!(f, "Private"),
        }
    }
}