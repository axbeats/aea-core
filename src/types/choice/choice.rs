use crate::*;

pub type ChoiceId = u64;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Choice {
    pub id: ChoiceId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub kind: ChoiceKind,
    pub video: VideoHash,
    pub image: ImageHash,
    pub description: String,
    pub size: u8,
    pub max_vote_options: u8,
    pub initiated_at: u64,
}

impl Choice {
    pub fn from_input(id: ChoiceId, input: ChoiceInput) -> Self {
        Choice {
            id,
            dao_id: input.dao_id,
            group_id: input.group_id,
            kind: input.kind,
            video: input.video_hash,
            image: input.thumbnail_hash,
            description: input.description,
            size: input.initial_values.len() as u8,
            max_vote_options: input.max_vote_options,
            initiated_at: env::block_timestamp(),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ChoiceVoteCount {
    pub option_id: OptionId,
    pub count: u128,
}