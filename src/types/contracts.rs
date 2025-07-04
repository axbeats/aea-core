use crate::*;

// Dynamically generated contract IDs
pub const ACCOUNT_FACTORY_ID: &str = "aea4.testnet";
pub const ALGORITHM_ID: &str = "algorithm1.aea4.testnet";
pub const AUDIO_ID: &str = "audio1.aea4.testnet";
pub const BUSINESS_FACTORY_ID: &str = "business1.aea4.testnet";
pub const CALIBRATION_ID: &str = "calibration1.aea4.testnet";
pub const CHOICE_ID: &str = "choice1.aea4.testnet";
pub const CONTRACT_MANAGER_ID: &str = "contract_manager1.aea4.testnet";
pub const COURT_ID: &str = "court1.aea4.testnet";
pub const DAO_FACTORY_ID: &str = "dao1.aea4.testnet";
pub const FT_FACTORY_ID: &str = "ft1.aea4.testnet";
pub const FT_MINT_ID: &str = "mint1.aea4.testnet";
pub const FT_SALE_FACTORY_ID: &str = "fundraise1.aea4.testnet";
pub const GROUP_ID: &str = "group1.aea4.testnet";
pub const HUMANITY_ID: &str = "humanity1.aea4.testnet";
pub const MARKETPLACE_ID: &str = "marketplace1.aea4.testnet";
pub const NFT_ID: &str = "nft1.aea4.testnet";
pub const PRODUCT_ID: &str = "product1.aea4.testnet";
pub const PROFILE_ID: &str = "profile2.aea4.testnet";
pub const PROPOSAL_ID: &str = "proposal1.aea4.testnet";
pub const STAKING_FACTORY_ID: &str = "staking1.aea4.testnet";
pub const TOKEN_ID: &str = "aea1.ft1.aea4.testnet";
pub const VALUE_ID: &str = "value1.aea4.testnet";
pub const VIDEO_ID: &str = "video2.aea4.testnet";

/// Enum of all recognized contract IDs. Each variant corresponds to a constant above.
#[derive(Debug, Clone)]
pub enum AeaContract {
    AccountFactory,
    Algorithm,
    Audio,
    BusinessFactory,
    Calibration,
    Choice,
    ContractManager,
    Court,
    DaoFactory,
    FtFactory,
    FtMint,
    FtSaleFactory,
    Group,
    Humanity,
    Marketplace,
    Nft,
    Product,
    Profile,
    Proposal,
    StakingFactory,
    Token,
    Value,
    Video,
}

impl AeaContract {
    /// Return the actual NEAR account ID (&str) corresponding to the variant.
    pub fn value(&self) -> &'static str {
        match self {
            AeaContract::AccountFactory => ACCOUNT_FACTORY_ID,
            AeaContract::Algorithm => ALGORITHM_ID,
            AeaContract::Audio => AUDIO_ID,
            AeaContract::BusinessFactory => BUSINESS_FACTORY_ID,
            AeaContract::Calibration => CALIBRATION_ID,
            AeaContract::Choice => CHOICE_ID,
            AeaContract::ContractManager => CONTRACT_MANAGER_ID,
            AeaContract::Court => COURT_ID,
            AeaContract::DaoFactory => DAO_FACTORY_ID,
            AeaContract::FtFactory => FT_FACTORY_ID,
            AeaContract::FtMint => FT_MINT_ID,
            AeaContract::FtSaleFactory => FT_SALE_FACTORY_ID,
            AeaContract::Group => GROUP_ID,
            AeaContract::Humanity => HUMANITY_ID,
            AeaContract::Marketplace => MARKETPLACE_ID,
            AeaContract::Nft => NFT_ID,
            AeaContract::Product => PRODUCT_ID,
            AeaContract::Profile => PROFILE_ID,
            AeaContract::Proposal => PROPOSAL_ID,
            AeaContract::StakingFactory => STAKING_FACTORY_ID,
            AeaContract::Token => TOKEN_ID,
            AeaContract::Value => VALUE_ID,
            AeaContract::Video => VIDEO_ID,
        }
    }
}

/// Converts a string (the actual NEAR account ID) into a `Contract` (if it matches).
pub fn str_to_contract_id(s: &str) -> Option<AeaContract> {
    match s {
        ACCOUNT_FACTORY_ID => Some(AeaContract::AccountFactory),
        ALGORITHM_ID => Some(AeaContract::Algorithm),
        AUDIO_ID => Some(AeaContract::Audio),
        BUSINESS_FACTORY_ID => Some(AeaContract::BusinessFactory),
        CALIBRATION_ID => Some(AeaContract::Calibration),
        CHOICE_ID => Some(AeaContract::Choice),
        CONTRACT_MANAGER_ID => Some(AeaContract::ContractManager),
        COURT_ID => Some(AeaContract::Court),
        DAO_FACTORY_ID => Some(AeaContract::DaoFactory),
        FT_FACTORY_ID => Some(AeaContract::FtFactory),
        FT_MINT_ID => Some(AeaContract::FtMint),
        FT_SALE_FACTORY_ID => Some(AeaContract::FtSaleFactory),
        GROUP_ID => Some(AeaContract::Group),
        HUMANITY_ID => Some(AeaContract::Humanity),
        MARKETPLACE_ID => Some(AeaContract::Marketplace),
        NFT_ID => Some(AeaContract::Nft),
        PRODUCT_ID => Some(AeaContract::Product),
        PROFILE_ID => Some(AeaContract::Profile),
        PROPOSAL_ID => Some(AeaContract::Proposal),
        STAKING_FACTORY_ID => Some(AeaContract::StakingFactory),
        TOKEN_ID => Some(AeaContract::Token),
        VALUE_ID => Some(AeaContract::Value),
        VIDEO_ID => Some(AeaContract::Video),
        _ => None,
    }
}

