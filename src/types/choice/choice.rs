use crate::*;

pub type ChoiceId = u64;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Choice {
    pub id: ChoiceId,
    pub video_id: VideoId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub reference: ChoiceReference,
    pub kind: ChoiceKind,
    pub max_vote_options: u8,
    pub elected_candidates: Vec<CandidateId>, // Might not need this here - Dec 2 2024
    pub initiated_at: TimestampNanoSeconds,
}

impl Choice {
    pub fn from_input(id: ChoiceId, video_id: VideoId, input: ChoiceInput) -> Self {
        Choice {
            id,
            video_id,
            dao_id: input.dao_id,
            group_id: input.group_id,
            reference: input.reference,
            kind: input.kind,
            elected_candidates: input.initial_candidates,
            max_vote_options: input.max_vote_options,
            initiated_at: env::block_timestamp(),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ChoiceVoteCount {
    pub option_id: CandidateId,
    pub count: u128,
}