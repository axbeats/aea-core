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

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct DaoInitArgs {
    pub dao_input: DaoInput,
    pub metadata: ContractMetadata,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct FTSaleInput {
    pub owner_id: AccountId,
    pub sale_ft_id: ContractId,
    pub payment_ft_id: ContractId,
    pub price_per_token: U128,
    pub tokens_for_sale: U128,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct FTSaleConfig {
    pub owner_id: AccountId,
    pub sale_ft_id: ContractId,
    pub payment_ft_id: ContractId,
    pub price_per_token: U128,
    pub tokens_for_sale: U128,
    pub metadata: ContractMetadata,
}