// use crate::*;

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
// #[serde(crate = "near_sdk::serde")]
// pub enum VectorIds {
//     Int8(Vec<Int8Id>),
//     Int16(Vec<Int16Id>),
//     Int32(Vec<Int32Id>),
//     Int64(Vec<Int64Id>),
//     Int128(Vec<Int128Id>),
//     UInt8(Vec<UInt8Id>),
//     UInt16(Vec<UInt16Id>),
//     UInt32(Vec<UInt32Id>),
//     UInt64(Vec<UInt64Id>),
//     UInt128(Vec<UInt128Id>),
//     Float32(Vec<Float32Id>),
//     Float64(Vec<Float64Id>),
//     String(Vec<StringId>),
//     Bool(Vec<BoolId>),
//     Bytes(Vec<BytesId>),
//     Set(Vec<SetId>),
//     Vector(Vec<Box<VectorIds>>),
//     Map(Vec<MapId>),
//     Weights(Vec<WeightId>),
//     Distribution(Vec<DistributionId>),
//     Group(Vec<GroupId>),
// }

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub enum VectorInput {
//     Int8(Vec<i8>),
//     Int16(Vec<i16>),
//     Int32(Vec<i32>),
//     Int64(Vec<i64>),
//     Int128(Vec<i128>),
//     UInt8(Vec<u8>),
//     UInt16(Vec<u16>),
//     UInt32(Vec<u32>),
//     UInt64(Vec<u64>),
//     UInt128(Vec<u128>),
//     Float32(Vec<f32>),
//     Float64(Vec<f64>),
//     String(Vec<String>),
//     Bool(Vec<bool>),
//     Bytes(Vec<Vec<u8>>),
//     Vector(Vec<VectorInput>),
//     Set(Vec<SetInput>),
//     Map(Vec<StringMapInput>),
//     Weights(Vec<WeightInput>),
//     Distribution(Vec<DistributionInput>),
//     Group(Vec<GroupInput>),
// }