use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum ViolationTime {
    PointInTime(TimestampMilliSeconds),
    Range {
        start: TimestampMilliSeconds,
        end: TimestampMilliSeconds,
    },
    Unknown,
}