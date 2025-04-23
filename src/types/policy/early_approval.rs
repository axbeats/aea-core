use crate::*;

// I built this file as an idea, but haven't implemented it - Apr 16 2025
// Meant to require early_threshold and early_quorum together
// Alternative is more flexible
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum EarlyApprove {
    No,
    Yes(EarlyApproval),
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct EarlyApproval {
    pub early_threshold: Option<u8>,
    pub early_quorum: Option<u8>,
}