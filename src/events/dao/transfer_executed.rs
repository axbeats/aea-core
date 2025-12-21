use super::*;

/// Event emitted when the DAO executes a transfer via an approved proposal
#[near(serializers = [json])]
#[derive(Debug)]
pub struct TransferExecutedEvent {
    /// The DAO that executed the transfer
    pub dao_id: AccountId,
    /// Recipient of the transfer
    pub receiver_id: AccountId,
    /// Amount transferred
    pub amount: U128,
    /// Token contract ID (None for NEAR transfers)
    pub token_id: Option<AccountId>,
    /// Whether this was a ft_transfer_call (with msg)
    pub is_transfer_call: bool,
    /// Timestamp of execution
    pub timestamp: u64,
}

impl TransferExecutedEvent {
    pub fn emit(self) {
        let event = DaoEvent::new(DaoEventKind::TransferExecuted(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for TransferExecutedEvent {
    fn event_kind(&self) -> &str {
        "transfer_executed"
    }
}
