use crate::*;

pub type MintStreamId = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MintStream {
    pub id: MintStreamId,
    pub interactions: HashSet<MintInteractionId>,
    pub percentage: YoctoNumber,
    pub current_weights: HashMap<AccountId, u128>,
    pub carryover_negative_weights: HashMap<AccountId, u128>,
    pub last_mint_timestamp: TimestampNanoSeconds,
}
