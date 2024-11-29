use crate::*;

pub type TokenContractId = ContractId;
pub type ContractId = AccountId;
pub type StakingId = ContractId;

pub const ACCOUNT_FACTORY_ID: &str = "aea1.testnet";
pub const ALGORITHM_ID: &str = "algorithm3.aea1.testnet";
pub const AUDIO_ID: &str = "audio1.aea1.testnet";
pub const BUSINESS_FACTORY_ID: &str = "business1.aea1.testnet";
pub const CALIBRATION_ID: &str = "calibration1.aea1.testnet";
pub const CHOICE_ID: &str = "choice1.aea1.testnet";
pub const COURT_ID: &str = "court1.aea1.testnet";
pub const DAO_FACTORY_ID: &str = "dao1.aea1.testnet";
pub const GROUP_ID: &str = "group1.aea1.testnet";
pub const FT_ID: &str = "token1.aea1.testnet";
pub const FT_MINT_ID: &str = "mint1.aea1.testnet";
pub const NFT_ID: &str = "nft1.aea1.testnet";
pub const MARKETPLACE_ID: &str = "marketplace1.aea1.testnet";
pub const MANAGER_ID: &str = "manager1.aea1.testnet";
pub const PRODUCT_ID: &str = "product1.aea1.testnet";
pub const PROFILE_ID: &str = "user5.aea1.testnet";
pub const PROPOSAL_ID: &str = "proposal1.aea1.testnet";
pub const STAKING_FACTORY_ID: &str = "staking1.aea1.testnet";
pub const TOKEN_FACTORY_ID: &str = "token1.aea1.testnet";
pub const TOKEN_MINTER_ID: &str = "minter1.aea1.testnet";
pub const VALUE_ID: &str = "value1.aea1.testnet";
pub const VIDEO_ID: &str = "video1.aea1.testnet";

pub const CALLBACK_GAS: Gas = Gas::from_gas(5_000_000_000_000);