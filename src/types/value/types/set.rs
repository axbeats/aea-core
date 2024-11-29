// use crate::*;

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub enum SetIds {
//     Int8(HashSet<Int8Id>),
//     Int16(HashSet<Int16Id>),
//     Int32(HashSet<Int32Id>),
//     Int64(HashSet<Int64Id>),
//     Int128(HashSet<Int128Id>),
//     UInt8(HashSet<UInt8Id>),
//     UInt16(HashSet<UInt16Id>),
//     UInt32(HashSet<UInt32Id>),
//     UInt64(HashSet<UInt64Id>),
//     UInt128(HashSet<UInt128Id>),
//     Float32(HashSet<Float32Id>),
//     Float64(HashSet<Float64Id>),
//     String(HashSet<StringId>),
//     Bool(HashSet<BoolId>),
//     Bytes(HashSet<BytesId>),
// }

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub enum SetInput {
//     Int8(HashSet<i8>),
//     Int16(HashSet<i16>),
//     Int32(HashSet<i32>),
//     Int64(HashSet<i64>),
//     Int128(HashSet<i128>),
//     UInt8(HashSet<u8>),
//     UInt16(HashSet<u16>),
//     UInt32(HashSet<u32>),
//     UInt64(HashSet<u64>),
//     UInt128(HashSet<u128>),
//     String(HashSet<String>),
//     Bool(HashSet<bool>),
//     Bytes(HashSet<Vec<u8>>),
// }