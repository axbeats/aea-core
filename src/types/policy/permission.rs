use crate::*;

pub type PermissionString = String;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct ProposalPermission {
    pub create: bool,
    pub vote: bool,
    pub custom_policy: Option<CustomPolicy>,
}

impl Default for ProposalPermission {
    fn default() -> Self {
        Self {
            create: true,
            vote: true,
            custom_policy: None,
        }
    }
}