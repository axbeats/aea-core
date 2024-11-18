use crate::*;

pub type ProposalId = u64;
pub type CurrentStage = u8; // 1 based index

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    pub id: u64,
    pub video_id: VideoId,
    pub dao_id: AccountId,
    pub proposer_id: AccountId,
    pub proposer_group_id: GroupId,
    pub kind: ProposalKind,
    pub voting_sessions: Vec<ProposalGroupVotingSession>,
    pub status: ProposalStatus,
    pub submission_time: TimestampNanoSeconds,
    pub bond: u128,
}

impl Proposal {
    pub fn from_input(id: ProposalId, video_id: VideoId, input: ProposalInput) -> Self {
        Self {
            id,
            video_id,
            dao_id: input.dao_id,
            proposer_id: env::predecessor_account_id(),
            proposer_group_id: input.group_id,
            kind: input.kind,
            status: ProposalStatus::Initializing, // Will update later 
            voting_sessions: Vec::new(), // Will update later
            submission_time: env::block_timestamp(),
            bond: 0,
        }
    }
}

impl Proposal {

    pub fn get_current_voting_group(&self) -> GroupId {
        self.voting_sessions[self.status.get_voting_stage().unwrap() as usize - 1].group_id.clone()
    }

    // Method to advance the proposal stage and update the current and next group's voting status
    pub fn advance_stage(&mut self, new_status: GroupVoteStatus) {
        // Find the current voting session by checking the group status
        if let Some(current_index) = self.voting_sessions.iter().position(|session| session.status == GroupVoteStatus::VoteOpen) {
            // Update the current group's voting status
            if let Some(current_session) = self.voting_sessions.get_mut(current_index) {
                current_session.status = new_status;
            }

            // Move to the next group in the voting order, if there is one
            if let Some(next_session) = self.voting_sessions.get_mut(current_index + 1) {
                // Set the next group's voting status to `VotingOpen` and update the start time
                next_session.status = GroupVoteStatus::VoteOpen;
                next_session.start_time = Some(env::block_timestamp());
            } else {
                // If no next session, the voting has completed
                self.status.advance_stage();  // Possibly set to final or complete status here
            }
        }
    }

}