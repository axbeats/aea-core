use crate::*;

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub enum TypeAndValue {
//     AccountId(AccountId),
//     VideoId(VideoId),
//     Int8(i8),
//     Int16(i16),
//     Int32(i32),
//     Int64(i64),
//     Int128(i128),
//     UInt8(u8),
//     UInt16(u16),
//     UInt32(u32),
//     UInt64(u64),
//     UInt128(u128),
//     Float32(f32),
//     Float64(f64),
//     String(String),
//     Bool(bool),
//     Bytes(Vec<u8>),
//     Numbers(NumbersType),
//     Words(WordsType),
//     Sum100(DistributionType),
//     Group(HashSet<AccountId>),
// }


// #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub struct SetType {
//     pub elements: HashSet<TypeId>,
// }

// #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub struct MapType {
//     pub elements: HashMap<String, TypeId>,
// }



#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NumbersType {
    pub elements: HashMap<String, u128>,
}



#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct WordsType {
    pub elements: HashSet<String>,
}






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