use std::fmt;

use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
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