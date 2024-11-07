use crate::*;

pub type IndictmentId = u64;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Indictment {
    pub id: IndictmentId,
    pub accusation_ids: Vec<AccusationId>,
    pub law_id: LawId,
    pub dao_id: DaoId,
    pub accused_id: AccountId,
    pub evidence: Vec<Evidence>,
    pub status: IndictmentStatus,
    pub violation_time: ViolationTime,
    pub initiated_at: TimestampSeconds,
}
