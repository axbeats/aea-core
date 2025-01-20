use crate::*;

pub type DefaultBond = u128;
pub type AttachedBond = u128;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum ProposalPolicyKind {
    Threshold(u8),
    Quorum(u8),
    EarlyThreshold(u8),
    EarlyQuorum(u8),
    Bond(u128),
    VotingPeriod(u64),
}

impl ProposalPolicyKind {
    pub fn to_policy_label(&self) -> &'static str {
        match self {
            ProposalPolicyKind::Threshold(_) => "threshold",
            ProposalPolicyKind::Quorum(_) => "quorum",
            ProposalPolicyKind::EarlyThreshold(_) => "early_threshold",
            ProposalPolicyKind::EarlyQuorum(_) => "early_quorum",
            ProposalPolicyKind::Bond(_) => "bond",
            ProposalPolicyKind::VotingPeriod(_) => "voting_period",
        }
    }

    pub fn valid_value(&self) -> bool {
        match self {
            ProposalPolicyKind::Threshold(threshold) => *threshold <= 100,
            ProposalPolicyKind::Quorum(quorum) => *quorum <= 100,
            ProposalPolicyKind::EarlyThreshold(early_threshold) => *early_threshold <= 100,
            ProposalPolicyKind::EarlyQuorum(early_quorum) => *early_quorum <= 100,
            ProposalPolicyKind::Bond(_) => true, // Assuming any non-negative u128 is valid
            ProposalPolicyKind::VotingPeriod(_) => true, // Assuming any non-negative u64 is valid
        }
    }
}

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
// #[serde(crate = "near_sdk::serde")]
// pub struct ProposalPolicyInput {
//     pub value: ProposalPolicyKind, // Default vote policy. Used when given proposal kind doesn't have custom policy.
//     pub amendment_method: VoteMethod,
// }

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Policy {
    pub bond: u128,
    pub voting_period: u64,
    pub threshold: u8,
    pub early_threshold: Option<u8>,
    pub quorum: u8,
    pub early_quorum: Option<u8>,
}

impl Policy {
    pub fn apply_custom_policy(mut self, custom_policy: CustomPolicy) -> Self {
        if let Some(bond) = custom_policy.bond {
            self.bond = bond;
        }
        if let Some(voting_period) = custom_policy.voting_period {
            self.voting_period = voting_period;
        }
        if let Some(threshold) = custom_policy.threshold {
            self.threshold = threshold;
        }
        if let Some(early_threshold) = custom_policy.early_threshold {
            self.early_threshold = Some(early_threshold);
        }
        if let Some(quorum) = custom_policy.quorum {
            self.quorum = quorum;
        }
        if let Some(early_quorum) = custom_policy.early_quorum {
            self.early_quorum = Some(early_quorum);
        }

        self
    }
}


#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct CustomPolicy {
    pub bond: Option<u128>,
    pub voting_period: Option<u64>,
    pub threshold: Option<u8>,
    pub early_threshold: Option<u8>,
    pub quorum: Option<u8>,
    pub early_quorum: Option<u8>,
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


impl Default for CustomPolicy {
    fn default() -> Self {
        CustomPolicy {
            bond: None,
            voting_period: None,
            threshold: None,
            early_threshold: None,
            quorum: None,
            early_quorum: None,
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct CustomPolicyInput {
    pub group_id: GroupId,
    pub proposal_kind: ProposalKindString,
    pub policy_kind: ProposalPolicyKind,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum PolicyKind {
    Default(ProposalPolicyKind),
    Custom(CustomPolicyInput)
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct GroupVotePolicy {
    pub group_id: GroupId,
    pub group_size: u64,
    pub proposal_kind: ProposalKindString,
    pub voting_period: u64,
    pub threshold: u8,
    pub early_threshold: Option<u8>,
    pub quorum: u8,
    pub early_quorum: Option<u8>,
}