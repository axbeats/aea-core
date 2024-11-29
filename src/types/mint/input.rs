use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FTMintInput {
    pub interactions: Vec<MintInteraction>,
    pub streams: Vec<MintStream>,
    pub token_id: ContractId,
    pub total_minting_rate_per_second: YoctoNumber,
    pub mint_account_limit: u32,
    pub mint_period_limit: TimestampSeconds,
}