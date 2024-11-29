// use crate::*;

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub enum StringMapIds {
//     Int8(HashMap<String, Int8Id>),
//     Int16(HashMap<String, Int16Id>),
//     Int32(HashMap<String, Int32Id>),
//     Int64(HashMap<String, Int64Id>),
//     Int128(HashMap<String, Int128Id>),
//     UInt8(HashMap<String, UInt8Id>),
//     UInt16(HashMap<String, UInt16Id>),
//     UInt32(HashMap<String, UInt32Id>),
//     UInt64(HashMap<String, UInt64Id>),
//     UInt128(HashMap<String, UInt128Id>),
//     Float32(HashMap<String, Float32Id>),
//     Float64(HashMap<String, Float64Id>),
//     String(HashMap<String, StringId>),
//     Bool(HashMap<String, BoolId>),
//     Bytes(HashMap<String, BytesId>),
//     Set(HashMap<String, SetId>),
//     Vector(HashMap<String, VectorIds>),
//     Map(HashMap<String, Box<StringMapIds>>),
//     Weights(HashMap<String, WeightId>),
//     Distribution(HashMap<String, DistributionId>),
//     Group(HashMap<String, GroupId>),
// }

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub enum StringMapInput {
//     Int8(HashMap<String, i8>),
//     Int16(HashMap<String, i16>),
//     Int32(HashMap<String, i32>),
//     Int64(HashMap<String, i64>),
//     Int128(HashMap<String, i128>),
//     UInt8(HashMap<String, u8>),
//     UInt16(HashMap<String, u16>),
//     UInt32(HashMap<String, u32>),
//     UInt64(HashMap<String, u64>),
//     UInt128(HashMap<String, u128>),
//     Float32(HashMap<String, f32>),
//     Float64(HashMap<String, f64>),
//     String(HashMap<String, String>),
//     Bool(HashMap<String, bool>),
//     Bytes(HashMap<String, Vec<u8>>),
//     Vector(HashMap<String, VectorInput>),
//     Set(HashMap<String, SetInput>),
//     Map(HashMap<String, StringMapInput>),
//     Weights(HashMap<String, WeightInput>),
//     Distribution(HashMap<String, DistributionInput>),
//     Group(HashMap<String, GroupInput>),
// }

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub enum AccountMapIds {
//     Int8(HashMap<AccountId, Int8Id>),
//     Int16(HashMap<AccountId, Int16Id>),
//     Int32(HashMap<AccountId, Int32Id>),
//     Int64(HashMap<AccountId, Int64Id>),
//     Int128(HashMap<AccountId, Int128Id>),
//     UInt8(HashMap<AccountId, UInt8Id>),
//     UInt16(HashMap<AccountId, UInt16Id>),
//     UInt32(HashMap<AccountId, UInt32Id>),
//     UInt64(HashMap<AccountId, UInt64Id>),
//     UInt128(HashMap<AccountId, UInt128Id>),
//     Float32(HashMap<AccountId, Float32Id>),
//     Float64(HashMap<AccountId, Float64Id>),
//     String(HashMap<AccountId, StringId>),
//     Bool(HashMap<AccountId, BoolId>),
//     Bytes(HashMap<AccountId, BytesId>),
//     Set(HashMap<AccountId, SetId>),
//     Vector(HashMap<AccountId, VectorIds>),
//     Map(HashMap<AccountId, Box<AccountMapIds>>),
//     Weights(HashMap<AccountId, WeightId>),
//     Distribution(HashMap<AccountId, DistributionId>),
//     Group(HashMap<AccountId, GroupId>),
// }

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub enum AccountMapInput {
//     Int8(HashMap<AccountId, i8>),
//     Int16(HashMap<AccountId, i16>),
//     Int32(HashMap<AccountId, i32>),
//     Int64(HashMap<AccountId, i64>),
//     Int128(HashMap<AccountId, i128>),
//     UInt8(HashMap<AccountId, u8>),
//     UInt16(HashMap<AccountId, u16>),
//     UInt32(HashMap<AccountId, u32>),
//     UInt64(HashMap<AccountId, u64>),
//     UInt128(HashMap<AccountId, u128>),
//     Float32(HashMap<AccountId, f32>),
//     Float64(HashMap<AccountId, f64>),
//     String(HashMap<AccountId, String>),
//     Bool(HashMap<AccountId, bool>),
//     Bytes(HashMap<AccountId, Vec<u8>>),
//     Vector(HashMap<AccountId, VectorInput>),
//     Set(HashMap<AccountId, SetInput>),
//     Map(HashMap<AccountId, AccountMapInput>),
//     Weights(HashMap<AccountId, WeightInput>),
//     Distribution(HashMap<AccountId, DistributionInput>),
//     Group(HashMap<AccountId, GroupInput>),
// }

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub enum UInt64MapIds {
//     Int8(HashMap<u64, Int8Id>),
//     Int16(HashMap<u64, Int16Id>),
//     Int32(HashMap<u64, Int32Id>),
//     Int64(HashMap<u64, Int64Id>),
//     Int128(HashMap<u64, Int128Id>),
//     UInt8(HashMap<u64, UInt8Id>),
//     UInt16(HashMap<u64, UInt16Id>),
//     UInt32(HashMap<u64, UInt32Id>),
//     UInt64(HashMap<u64, UInt64Id>),
//     UInt128(HashMap<u64, UInt128Id>),
//     Float32(HashMap<u64, Float32Id>),
//     Float64(HashMap<u64, Float64Id>),
//     String(HashMap<u64, StringId>),
//     Bool(HashMap<u64, BoolId>),
//     Bytes(HashMap<u64, BytesId>),
//     Set(HashMap<u64, SetId>),
//     Vector(HashMap<u64, VectorIds>),
//     Map(HashMap<u64, Box<UInt64MapIds>>),
//     Weights(HashMap<u64, WeightId>),
//     Distribution(HashMap<u64, DistributionId>),
//     Group(HashMap<u64, GroupId>),
// }

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub enum UInt64MapInput {
//     Int8(HashMap<u64, i8>),
//     Int16(HashMap<u64, i16>),
//     Int32(HashMap<u64, i32>),
//     Int64(HashMap<u64, i64>),
//     Int128(HashMap<u64, i128>),
//     UInt8(HashMap<u64, u8>),
//     UInt16(HashMap<u64, u16>),
//     UInt32(HashMap<u64, u32>),
//     UInt64(HashMap<u64, u64>),
//     UInt128(HashMap<u64, u128>),
//     Float32(HashMap<u64, f32>),
//     Float64(HashMap<u64, f64>),
//     String(HashMap<u64, String>),
//     Bool(HashMap<u64, bool>),
//     Bytes(HashMap<u64, Vec<u8>>),
//     Vector(HashMap<u64, VectorInput>),
//     Set(HashMap<u64, SetInput>),
//     Map(HashMap<u64, UInt64MapInput>),
//     Weights(HashMap<u64, WeightInput>),
//     Distribution(HashMap<u64, DistributionInput>),
//     Group(HashMap<u64, GroupInput>),
// }
