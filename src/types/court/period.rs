use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ViolationTime {
    PointInTime(TimestampMilliSeconds),
    Range {
        start: TimestampMilliSeconds,
        end: TimestampMilliSeconds,
    },
    Unknown,
}