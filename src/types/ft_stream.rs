use crate::*;

pub type FTStreamId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct FTStream {
    pub from: AccountId,
    pub to: AccountId,
    pub rate: u128, // tokens per second
    pub start_time: u64,
    pub end_time: Option<u64>, // When stream expires (None = perpetual)
    pub active: bool, // true = active, false = cancelled
    pub dependent_stream: Option<FTStreamId>, // Bidirectional link for cascading cancels
    pub notify_on_cancel: Option<AccountId>, // Contract to notify when stream is cancelled
}

/// Stream status (calculated on-demand, not stored)
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamStatus {
    Scheduled,  // start_time > now
    Active,     // start_time <= now && (end_time.is_none() || end_time > now)
    Expired,    // end_time <= now
    Cancelled,  // active == false
}

impl FTStream {
    /// Calculate current status of the stream
    pub fn get_status(&self, now: u64) -> StreamStatus {
        if !self.active {
            return StreamStatus::Cancelled;
        }

        if self.start_time > now {
            return StreamStatus::Scheduled;
        }

        if let Some(end_time) = self.end_time {
            if end_time <= now {
                return StreamStatus::Expired;
            }
        }

        StreamStatus::Active
    }
}

/// Response from ft_on_stream callback
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct StreamAcceptance {
    /// Whether the stream is accepted
    pub accepted: bool,
    /// If Some, create a linked stream forwarding to this recipient at this rate
    pub forward_to: Option<(AccountId, u128)>,
}