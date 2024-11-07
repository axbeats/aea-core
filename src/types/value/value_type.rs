use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub enum ValueType {
    AccountId(AccountId),
    Boolean(bool),
    Bytes(Vec<u8>),
    // Float(f64),
    Int64(i64),
    String(String),
    UInt8(u8),
    UInt64(u64),
    UInt128(u128),
    Video(VideoHash), // This should be VideoId? - Oct 21 2024
    Image(ImageHash),
    Audio(AudioHash),
    Location(String),
    Date(u64),
    Time(u64),
    Distribution(Distribution),
}

impl ValueType {
    pub fn matches(&self, validation: &ValidationType) -> bool {
        match (self, validation) {
            (ValueType::AccountId(_), ValidationType::AccountId(_)) => true,
            (ValueType::Boolean(_), ValidationType::Boolean(_)) => true,
            (ValueType::Bytes(_), ValidationType::Bytes(_)) => true,
            (ValueType::Int64(_), ValidationType::Int64(_)) => true,
            (ValueType::String(_), ValidationType::String(_)) => true,
            (ValueType::UInt8(_), ValidationType::UInt8(_)) => true,
            (ValueType::UInt64(_), ValidationType::UInt64(_)) => true,
            (ValueType::UInt128(_), ValidationType::UInt128(_)) => true,
            (ValueType::Video(_), ValidationType::Video(_)) => true,
            (ValueType::Image(_), ValidationType::Image(_)) => true,
            (ValueType::Audio(_), ValidationType::Audio(_)) => true,
            (ValueType::Location(_), ValidationType::Location(_)) => true,
            (ValueType::Date(_), ValidationType::Date(_)) => true,
            (ValueType::Time(_), ValidationType::Time(_)) => true,
            _ => false,
        }
    }
}