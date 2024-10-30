use crate::*;

/// Metadata for a contract on the blockchain.
///
/// This struct holds various metadata fields that provide information about the contract,
/// including its version, name, symbol, icon, and references to additional resources.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
// Specify that Serde should use the `near_sdk::serde` crate for serialization.
#[serde(crate = "near_sdk::serde")]
pub struct ContractMetadata {
    pub spec: String,                 // Required, specifies the version of the contract metadata, e.g., "contract-1.0.0".
    pub name: String,                 // Required, name of the contract, e.g., "MyContract".
    pub symbol: Option<String>,               // Required, symbol representing the contract, e.g., "MYC".
    pub icon_hash: Option<String>,    // Optional, Hash for the icon associated with the contract.
    pub base_uri: Option<String>,     // Optional, Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs.
    pub reference: Option<String>,    // Optional, URL to a JSON file with more information about the contract.
    pub reference_hash: Option<Base64VecU8>, // Optional, Base64-encoded sha256 hash of JSON from the reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    pub spec: String,              // required, essentially a version like "nft-1.0.0"
    pub name: String,              // required, ex. "Mosaics"
    pub symbol: String,            // required, ex. "MOSAIC"
    pub icon: Option<String>,      // Data URL
    pub base_uri: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
    pub reference: Option<String>, // URL to a JSON file with more info
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize, Debug)]
#[borsh(crate = "near_sdk::borsh")]
#[serde(crate = "near_sdk::serde")]
pub struct FungibleTokenMetadata {
    pub spec: String, // Should be ft-1.0.0 to indicate that a Fungible Token contract adheres to the current versions of this Metadata and the Fungible Token Core specs. This will allow consumers of the Fungible Token to know if they support the features of a given contract.
    pub name: String, // The human-readable name of the token.
    pub symbol: String, // The abbreviation, like wETH or AMPL.
    pub icon: Option<String>, // Icon of the fungible token.
    pub reference: Option<String>, // A link to a valid JSON file containing various keys offering supplementary details on the token 
    pub reference_hash: Option<Base64VecU8>, // The base64-encoded sha256 hash of the JSON file contained in the reference field. This is to guard against off-chain tampering.
    pub decimals: u8, // used in frontends to show the proper significant digits of a token. This concept is explained well in this OpenZeppelin post. https://docs.openzeppelin.com/contracts/3.x/erc20#a-note-on-decimals
}