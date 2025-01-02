use crate::*;

/// Function call arguments.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FunctionCall {
    pub contract_id: AccountId,
    pub method_name: String,
    pub args: Base64VecU8,
    pub deposit: U128,
    pub gas: U64,
}

// I think this will be expired, check later and delete if so - Aug 9 2024
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ActionVoteProposal {
    pub proposal_id: ProposalId,
    pub proposal_kind: ProposalKindString,
    pub voter_id: AccountId,
    pub group_id: GroupId,
    pub vote: Vote,
    pub weight_kind: WeightKind,
    pub current_stage: u8, // 1 based index
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ActionVoteChoice {
    pub governed_value_id: ValueId,
    pub voter_id: AccountId,
    pub group_id: GroupId,
    pub candidate: String,
    pub weight_kind: WeightKind,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Action {
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub kind: ActionKind,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ActionInput {
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    // pub default_policy: Policy,
    pub kind: ActionInputKind,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ActionInputKind {
    CreateProposal((ProposalInput, AttachedBond, DefaultBond)),
    VoteProposal((ProposalVoteInput, CurrentStage, Policy)),
    VoteChoice(ChoiceVoteInput),
    VoteCalibration(CalibrationVoteInput),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ActionKind {
    CreateProposal(ProposalInput),
    VoteProposal(ProposalVote, Policy, u128),
    VoteChoice(ChoiceVoteConfig),
    VoteCalibration(CalibrationVoteConfig),
}

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub enum ActionVote {
//     Proposal(ActionVoteProposal),
//     Choice(ActionVoteChoice),
// }

// impl ActionVote {

//     pub fn voter_id(&self) -> &AccountId {
//         match self {
//             ActionVote::Proposal(proposal) => &proposal.voter_id,
//             ActionVote::Choice(choice) => &choice.voter_id,
//         }
//     }

//     pub fn group_id(&self) -> &GroupId {
//         match self {
//             ActionVote::Proposal(proposal) => &proposal.group_id,
//             ActionVote::Choice(choice) => &choice.group_id,
//         }
//     }
// }
