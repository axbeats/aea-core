use super::*;

/// Event emitted when the DAO executes a function call via an approved proposal
#[near(serializers = [json])]
#[derive(Debug)]
pub struct FunctionCallExecutedEvent {
    /// The DAO that executed the function call
    pub dao_id: AccountId,
    /// Target contract that was called
    pub contract_id: AccountId,
    /// Method that was called
    pub method_name: String,
    /// Deposit sent with the call (in yoctoNEAR)
    pub deposit: U128,
    /// Gas allocated for the call
    pub gas: U64,
    /// Number of function calls in the batch
    pub batch_size: u32,
    /// Index of this call in the batch (0-based)
    pub batch_index: u32,
    /// Timestamp of execution
    pub timestamp: u64,
}

impl FunctionCallExecutedEvent {
    pub fn emit(self) {
        let event = DaoEvent::new(DaoEventKind::FunctionCallExecuted(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for FunctionCallExecutedEvent {
    fn event_kind(&self) -> &str {
        "function_call_executed"
    }
}
