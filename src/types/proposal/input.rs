use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalInput {
    pub dao_id: AccountId,
    pub group_id: GroupId,
    pub video_hash: String,
    pub thumbnail_hash: String,
    pub description: String,
    pub kind: ProposalKind,
    pub bond: u128,
}

// impl From<ProposalInput> for Proposal {
//     fn from(input: ProposalInput) -> Self {
//         Self {
//             id: 0, // Will update later 
//             dao_id: input.dao_id,
//             proposer_id: env::predecessor_account_id(),
//             group_id: input.group_id,
//             video: input.video_hash,
//             image: input.thumbnail_hash,
//             description: input.description,
//             kind: input.kind,
//             status: ProposalStatus::Initializing, // Will update later 
//             // current_voting_group: "".to_string(), // Will update later
//             current_voting_group: 0, // Will update later
//             voting_sequence: Vec::new(), // Will update later
//             submission_time: env::block_timestamp(),
//             bond: 0,
//         }
//     }
// }

impl From<ProposalInput> for Proposal {
    fn from(input: ProposalInput) -> Self {
        Self {
            id: 0, // Will update later 
            dao_id: input.dao_id,
            proposer_id: env::predecessor_account_id(),
            proposer_group_id: input.group_id,
            video: input.video_hash,
            image: input.thumbnail_hash,
            description: input.description,
            kind: input.kind,
            status: ProposalStatus::Initializing, // Will update later 
            // current_voting_group: "".to_string(), // Will update later
            voting_sessions: Vec::new(), // Will update later
            submission_time: env::block_timestamp(),
            bond: 0,
        }
    }
}