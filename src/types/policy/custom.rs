use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct CustomPolicy {
    pub bond: Option<U128>,
    pub voting_period: Option<u64>,
    pub threshold: Option<u8>,
    pub early_threshold: Option<u8>,
    pub quorum: Option<u8>,
    pub early_quorum: Option<u8>,
}

impl Default for CustomPolicy {
    fn default() -> Self {
        Self {
            bond: None,
            voting_period: None,
            threshold: None,
            early_threshold: None,
            quorum: None,
            early_quorum: None,
        }
    }
}

impl CustomPolicy {
    pub fn is_complete(&self) -> bool {
        self.bond.is_some()
            && self.voting_period.is_some()
            && self.threshold.is_some()
            && self.early_threshold.is_some()
            && self.quorum.is_some()
            && self.early_quorum.is_some()
    }
}

impl CustomPolicy {
    pub fn to_policy(&self) -> Policy {
        Policy {
            bond: self.bond.expect("CustomPolicy is not complete: missing bond"),
            voting_period: self.voting_period.expect("CustomPolicy is not complete: missing voting_period"),
            threshold: self.threshold.expect("CustomPolicy is not complete: missing threshold"),
            early_threshold: self.early_threshold,
            quorum: self.quorum.expect("CustomPolicy is not complete: missing quorum"),
            early_quorum: self.early_quorum,
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct CustomPolicyInput {
    pub role_id: RoleId,
    pub ability: ProposalAbility,
    pub policy: CustomPolicy,
}