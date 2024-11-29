use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct Int8Id(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct Int16Id(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct Int32Id(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct Int64Id(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct Int128Id(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct Float16Id(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct Float32Id(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct Float64Id(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct Float128Id(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct UInt8Id(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct UInt16Id(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct UInt32Id(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct UInt64Id(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct UInt128Id(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct StringId(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct BoolId(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct BytesId(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct VectorId(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct SetId(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct MapId(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct WeightId(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[serde(crate = "near_sdk::serde")]
pub struct DistributionId(pub u64);


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct LastIds {
    pub last_value_id: ValueId,
    pub last_string_id: StringId,
    pub last_i8_id: Int8Id,
    pub last_i16_id: Int16Id,
    pub last_i32_id: Int32Id,
    pub last_i64_id: Int64Id,
    pub last_i128_id: Int128Id,
    pub last_u8_id: UInt8Id,
    pub last_u16_id: UInt16Id,
    pub last_u32_id: UInt32Id,
    pub last_u64_id: UInt64Id,
    pub last_u128_id: UInt128Id,
    pub last_f32_id: Float32Id,
    pub last_f64_id: Float64Id,
    pub last_bool_id: BoolId,
    pub last_bytes_id: BytesId,
    pub last_vector_id: VectorId,
    pub last_set_id: SetId,
    pub last_map_id: MapId,
    pub last_weight_id: WeightId,
    pub last_distribution_id: DistributionId,
    pub last_group_id: GroupId,
}

impl Default for LastIds {
    fn default() -> Self {
        Self {
            last_value_id: 0,
            last_string_id: StringId(0),
            last_i8_id: Int8Id(0),
            last_i16_id: Int16Id(0),
            last_i32_id: Int32Id(0),
            last_i64_id: Int64Id(0),
            last_i128_id: Int128Id(0),
            last_u8_id: UInt8Id(0),
            last_u16_id: UInt16Id(0),
            last_u32_id: UInt32Id(0),
            last_u64_id: UInt64Id(0),
            last_u128_id: UInt128Id(0),
            last_f32_id: Float32Id(0),
            last_f64_id: Float64Id(0),
            last_bool_id: BoolId(0),
            last_bytes_id: BytesId(0),
            last_vector_id: VectorId(0),
            last_set_id: SetId(0),
            last_map_id: MapId(0),
            last_weight_id: WeightId(0),
            last_distribution_id: DistributionId(0),
            last_group_id: 0,
        }
    }
}
