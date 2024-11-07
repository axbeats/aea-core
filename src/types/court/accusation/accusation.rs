use crate::*;

pub type AccusationId = u64;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Accusation {
    pub id: AccusationId,
    pub law_id: LawId,
    pub dao_id: DaoId,
    pub accused_id: AccountId,
    pub accuser_id: AccountId,
    pub evidence: Evidence,
    pub violation_time: ViolationTime,
    pub reported_at: TimestampMilliSeconds,
}

impl Accusation {
    pub fn from_input(id: AccusationId, input: AccusationInput) -> Self {
        Accusation {
            id,
            law_id: input.law_id,
            dao_id: input.dao_id,
            accused_id: input.accused_id,
            accuser_id: env::signer_account_id(),
            evidence: input.evidence,
            violation_time: input.violation_time,
            reported_at: env::block_timestamp_ms(),
        }
    }
}
