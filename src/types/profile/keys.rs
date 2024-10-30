use crate::*;

pub type EthereumAddress = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PublicKeys {
    pub ed25519: String,
    pub secp256k1: String,
    pub ethereum_address: EthereumAddress,
}