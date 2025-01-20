use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct StakingConfig {
    pub token_id: ContractId,
    pub owner_id: AccountId,
    pub metadata: ContractMetadata,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct TokenConfig {
    pub owner_id: AccountId,
    pub mint_id: Option<AccountId>,
    pub total_supply: U128,
    pub metadata: FungibleTokenMetadata,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct TokenInput {
    pub name: String,
    pub symbol: String,
    pub icon: ImageHash,
    pub total_supply: u128,
    pub decimals: u8,
}

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
// #[serde(crate = "near_sdk::serde")]
// pub enum StakingInput {
//     Internal,
//     External(TokenId),
// }

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct DaoInitArgs {
    pub dao_input: DaoInput,
    pub metadata: ContractMetadata,
}