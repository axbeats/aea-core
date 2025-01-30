use crate::*;

// Dynamically generated contract IDs
pub const ACCOUNT_FACTORY_ID: &str = "aea4.testnet";
pub const ALGORITHM_ID: &str = "algorithm1.aea4.testnet";
pub const AUDIO_ID: &str = "audio1.aea4.testnet";
pub const BUSINESS_FACTORY_ID: &str = "business1.aea4.testnet";
pub const CALIBRATION_ID: &str = "calibration1.aea4.testnet";
pub const CHOICE_ID: &str = "choice1.aea4.testnet";
pub const COURT_ID: &str = "court1.aea4.testnet";
pub const DAO_FACTORY_ID: &str = "dao1.aea4.testnet";
pub const FT_FACTORY_ID: &str = "ft1.aea4.testnet";
pub const FT_MINT_ID: &str = "ft_mint1.aea4.testnet";
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
pub enum Contract {
    AccountFactory,
    Algorithm,
    Audio,
    BusinessFactory,
    Calibration,
    Choice,
    Court,
    DaoFactory,
    FtFactory,
    FtMint,
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

impl Contract {
    /// Return the actual NEAR account ID (&str) corresponding to the variant.
    pub fn value(&self) -> &'static str {
        match self {
            Contract::AccountFactory => ACCOUNT_FACTORY_ID,
            Contract::Algorithm => ALGORITHM_ID,
            Contract::Audio => AUDIO_ID,
            Contract::BusinessFactory => BUSINESS_FACTORY_ID,
            Contract::Calibration => CALIBRATION_ID,
            Contract::Choice => CHOICE_ID,
            Contract::Court => COURT_ID,
            Contract::DaoFactory => DAO_FACTORY_ID,
            Contract::FtFactory => FT_FACTORY_ID,
            Contract::FtMint => FT_MINT_ID,
            Contract::Group => GROUP_ID,
            Contract::Humanity => HUMANITY_ID,
            Contract::Marketplace => MARKETPLACE_ID,
            Contract::Nft => NFT_ID,
            Contract::Product => PRODUCT_ID,
            Contract::Profile => PROFILE_ID,
            Contract::Proposal => PROPOSAL_ID,
            Contract::StakingFactory => STAKING_FACTORY_ID,
            Contract::Token => TOKEN_ID,
            Contract::Value => VALUE_ID,
            Contract::Video => VIDEO_ID,
        }
    }
}

/// Converts a string (the actual NEAR account ID) into a `Contract` (if it matches).
pub fn str_to_contract_id(s: &str) -> Option<Contract> {
    match s {
        ACCOUNT_FACTORY_ID => Some(Contract::AccountFactory),
        ALGORITHM_ID => Some(Contract::Algorithm),
        AUDIO_ID => Some(Contract::Audio),
        BUSINESS_FACTORY_ID => Some(Contract::BusinessFactory),
        CALIBRATION_ID => Some(Contract::Calibration),
        CHOICE_ID => Some(Contract::Choice),
        COURT_ID => Some(Contract::Court),
        DAO_FACTORY_ID => Some(Contract::DaoFactory),
        FT_FACTORY_ID => Some(Contract::FtFactory),
        FT_MINT_ID => Some(Contract::FtMint),
        GROUP_ID => Some(Contract::Group),
        HUMANITY_ID => Some(Contract::Humanity),
        MARKETPLACE_ID => Some(Contract::Marketplace),
        NFT_ID => Some(Contract::Nft),
        PRODUCT_ID => Some(Contract::Product),
        PROFILE_ID => Some(Contract::Profile),
        PROPOSAL_ID => Some(Contract::Proposal),
        STAKING_FACTORY_ID => Some(Contract::StakingFactory),
        TOKEN_ID => Some(Contract::Token),
        VALUE_ID => Some(Contract::Value),
        VIDEO_ID => Some(Contract::Video),
        _ => None,
    }
}

