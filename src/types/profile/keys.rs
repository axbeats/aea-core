use crate::*;
use aea_macros::Generable;

pub type EthereumAddress = String;
pub type InboxId = String;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct PublicKeys {
    pub ed25519: String,
    pub secp256k1: String,
    pub ethereum_address: EthereumAddress,
    pub inbox_id: InboxId,
}

impl Default for PublicKeys {
    fn default() -> Self {
        Self {
            ed25519: "".to_string(),
            secp256k1: "".to_string(),
            ethereum_address: "".to_string(),
            inbox_id: "".to_string(),
        }
    }
}