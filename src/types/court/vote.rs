use crate::*;

pub type IndictmentVoteId = u64;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct IndictmentVote {
    pub id: IndictmentVoteId,
    pub dao_id: DaoId,
    pub law_id: LawId,
    pub indictment_id: IndictmentId,
    pub account_id: AccountId,
    pub group_id: GroupId,
    pub vote: IndictmentVoteKind,
    pub weight: u128,
    pub issued_at: TimestampMilliSeconds,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct IndictmentVoteInput {
    pub indictment_id: IndictmentId,
    pub group_id: GroupId,
    pub vote: IndictmentVoteKind,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct IndictmentVoteTally {
    pub innocent_count: u128,
    pub guilty_count: u128,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum IndictmentVoteKind {
    Innocent,
    Guilty,
}