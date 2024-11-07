use crate::*;

pub type DayCountUnix = u64;
pub type TimestampSeconds = u64;
pub type TimestampMilliSeconds = u64;
pub type TimestampNanoSeconds = u64;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum TimePeriod {
    Day(Option<u64>),
    Week(Option<u64>),
    Month(Option<u64>),
    Year(Option<u64>),
}