use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FTMintInput {
    // pub mint_streams: HashMap<String, MintStream>,
    pub mint_amount_per_second: YoctoNumber,
    pub mint_account_limit: u32,
    pub mint_period_limit: TimestampSeconds,
}