use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalStatus {
    Initializing,
    VotingStage(VotingStage), // Voting stage with current stage and final stage
    Approved,
    Rejected,
    Spam,
    Expired,
    Failed, // Proposal execution fails
}

impl ProposalStatus {
    pub fn get_voting_stage(&self) -> Option<u8> {
        if let ProposalStatus::VotingStage(voting_stage) = self {
            Some(voting_stage.current_stage)
        } else {
            None
        }
    }

    pub fn advance_stage(&mut self) {
        if let ProposalStatus::VotingStage(voting_stage) = self {
            if voting_stage.current_stage < voting_stage.final_stage {
                voting_stage.current_stage += 1;
            }   
        }
    }

    pub fn is_final_stage(&self) -> bool {
        match self {
            ProposalStatus::VotingStage(voting_stage) => voting_stage.current_stage == voting_stage.final_stage,
            _ => false,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct VotingStage {
    pub current_stage: u8, // 1 based index
    pub final_stage: u8, // 1 based index
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum GroupVoteStatus {
    Waiting,
    VoteOpen,
    VoteClosed(GroupVoteResult),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum GroupVoteResult {
    Approved,
    Rejected,
    Spam,
    Expired,
}