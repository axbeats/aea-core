use crate::*;

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub struct MintStream {
//     pub name: String,
//     pub percentage: YoctoNumber,
//     pub weights: HashMap<AccountId, u128>,
//     pub negative_weights: HashMap<AccountId, u128>,
//     pub last_mint_timestamp: TimestampNanoSeconds,
//     pub interactions: MintInteraction,
// }


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MintStream<T> {
    pub name: String,
    pub percentage: u16, // Percentage in basis points, e.g., 5000 for 50%
    pub accumulated_mint: u128,
    pub negative_weights: HashMap<AccountId, u128>,
    pub interactions: HashMap<AccountId, Vec<T>>, // Stores specific interaction type
    pub last_mint_timestamp: TimestampNanoSeconds,
}
