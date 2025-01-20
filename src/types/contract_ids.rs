use crate::*;
use dotenv::dotenv;
use std::env;

pub type TokenContractId = ContractId;
pub type ContractId = AccountId;
pub type StakingId = ContractId;

// Load .env file at runtime
fn load_env_var(key: &str, default: &str) -> String {
    dotenv().ok(); // Load environment variables from `contracts.env`
    env::var(key).unwrap_or_else(|_| default.to_string()) // Use default if key is not found
}

// Dynamically load contract IDs
lazy_static::lazy_static! {
    pub static ref ACCOUNT_FACTORY_ID: String = load_env_var("ACCOUNT_FACTORY_ID", "default.aea1.testnet");
    pub static ref ALGORITHM_ID: String = load_env_var("ALGORITHM_ID", "default.algorithm3.aea1.testnet");
    pub static ref AUDIO_ID: String = load_env_var("AUDIO_ID", "default.audio1.aea1.testnet");
    pub static ref BUSINESS_FACTORY_ID: String = load_env_var("BUSINESS_FACTORY_ID", "default.business1.aea1.testnet");
    pub static ref CALIBRATION_ID: String = load_env_var("CALIBRATION_ID", "default.calibration1.aea1.testnet");
    pub static ref CHOICE_ID: String = load_env_var("CHOICE_ID", "default.choice1.aea1.testnet");
    pub static ref COURT_ID: String = load_env_var("COURT_ID", "default.court1.aea1.testnet");
    pub static ref DAO_FACTORY_ID: String = load_env_var("DAO_FACTORY_ID", "default.dao1.aea1.testnet");
    pub static ref GROUP_ID: String = load_env_var("GROUP_ID", "default.group1.aea1.testnet");
    pub static ref FT_FACTORY_ID: String = load_env_var("FT_FACTORY_ID", "default.token1.aea1.testnet");
    pub static ref FT_MINT_ID: String = load_env_var("FT_MINT_ID", "default.mint1.aea1.testnet");
    pub static ref NFT_ID: String = load_env_var("NFT_ID", "default.nft1.aea1.testnet");
    pub static ref MARKETPLACE_ID: String = load_env_var("MARKETPLACE_ID", "default.marketplace1.aea1.testnet");
    pub static ref MANAGER_ID: String = load_env_var("MANAGER_ID", "default.manager1.aea1.testnet");
    pub static ref PRODUCT_ID: String = load_env_var("PRODUCT_ID", "default.product1.aea1.testnet");
    pub static ref PROFILE_ID: String = load_env_var("PROFILE_ID", "default.user5.aea1.testnet");
    pub static ref PROPOSAL_ID: String = load_env_var("PROPOSAL_ID", "default.proposal1.aea1.testnet");
    pub static ref STAKING_FACTORY_ID: String = load_env_var("STAKING_FACTORY_ID", "default.staking1.aea1.testnet");
    pub static ref TOKEN_FACTORY_ID: String = load_env_var("TOKEN_FACTORY_ID", "default.token1.aea1.testnet");
    pub static ref TOKEN_MINTER_ID: String = load_env_var("TOKEN_MINTER_ID", "default.minter1.aea1.testnet");
    pub static ref VALUE_ID: String = load_env_var("VALUE_ID", "default.value1.aea1.testnet");
    pub static ref VIDEO_ID: String = load_env_var("VIDEO_ID", "default.video1.aea1.testnet");
}

// pub const ACCOUNT_FACTORY_ID: &str = "aea2.testnet";
// pub const ALGORITHM_ID: &str = "algorithm1.aea2.testnet";
// pub const AUDIO_ID: &str = "audio1.aea2.testnet";
// pub const CALIBRATION_ID: &str = "calibration1.aea2.testnet";
// pub const CHOICE_ID: &str = "choice1.aea2.testnet";
// pub const COURT_ID: &str = "court1.aea2.testnet";
// pub const DAO_FACTORY_ID: &str = "dao1.aea2.testnet";
// pub const FT_FACTORY_ID: &str = "ft1.aea2.testnet";
// pub const FT_MINT_ID: &str = "ft_mint1.aea2.testnet";
// pub const GROUP_ID: &str = "group1.aea2.testnet";
// pub const HUMANITY_ID: &str = "humanity1.aea2.testnet";
// pub const MARKETPLACE_ID: &str = "marketplace1.aea2.testnet";
// pub const NFT_ID: &str = "nft1.aea2.testnet";
// pub const PROFILE_ID: &str = "profile1.aea2.testnet";
// pub const PROPOSAL_ID: &str = "proposal1.aea2.testnet";
// pub const STAKING_FACTORY_ID: &str = "staking1.aea2.testnet";
// pub const VALUE_ID: &str = "value1.aea2.testnet";
// pub const VIDEO_ID: &str = "video1.aea2.testnet";

pub const CALLBACK_GAS: Gas = Gas::from_gas(5_000_000_000_000);