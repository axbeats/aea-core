use near_sdk::{env, serde::{Deserialize, Serialize}, serde_json};

pub use self::account_factory::*;
pub use self::calibration::*;
pub use self::choice::*;
pub use self::contract::*;
pub use self::dao_factory::*;
pub use self::group::*;
pub use self::court::*;
pub use self::nft::*;
pub use self::profile::*;
pub use self::proposal::*;
pub use self::staking::*;
pub use self::staking_factory::*;
pub use self::standard::*;
pub use self::token::*;
pub use self::token_factory::*;
pub use self::value::*;
pub use self::video::*;

mod account_factory;
mod calibration;
mod choice;
mod contract;
mod dao_factory;
mod group;
mod court;
mod nft;
mod profile;
mod proposal;
mod staking;
mod staking_factory;
mod standard;
mod token;
mod token_factory;
mod value;
mod video;

pub trait EventKind {
    fn event_kind(&self) -> &str;
}
