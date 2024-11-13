use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AccusationInput {
    pub law_id: LawId,
    pub dao_id: DaoId,
    pub accused_id: AccountId,
    pub group_id: GroupId,
    pub evidence: Evidence,
    pub violation_time: ViolationTime,
    pub reported_at: TimestampMilliSeconds,
}