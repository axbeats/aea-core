use crate::*;

pub type EthereumAddress = String;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct PublicKeys {
    pub ed25519: String,
    pub secp256k1: String,
    pub ethereum_address: EthereumAddress,
}