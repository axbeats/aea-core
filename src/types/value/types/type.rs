use crate::*;




// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
// #[serde(crate = "near_sdk::serde")]
// pub enum TypeId {
//     Int8(Int8Id),
//     Int16(Int16Id),
//     Int32(Int32Id),
//     Int64(Int64Id),
//     Int128(Int128Id),
//     UInt8(UInt8Id),
//     UInt16(UInt16Id),
//     UInt32(UInt32Id),
//     UInt64(UInt64Id),
//     UInt128(UInt128Id),
//     Float32(Float32Id),
//     Float64(Float64Id),
//     String(StringId),
//     Bool(BoolId),
//     Bytes(BytesId),
//     Set(SetId),
//     Vector(VectorIds),
//     Map(MapId),
//     Weights(WeightId),
//     Distribution(DistributionId),
//     Group(GroupId),
// }

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub enum TypeInput {
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
//     Set(SetInput),
//     Vector(VectorInput),
//     Map(StringMapInput),
//     Weights(WeightInput),
//     Distribution(DistributionInput),
//     Group(GroupInput),
// }