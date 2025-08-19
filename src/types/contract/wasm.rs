use crate::*;

pub type Bytes = Vec<u8>;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct StoredWasm {
    pub wasm: Bytes,
    pub depositor: AccountId,
    pub balance: u128,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct WasmDeposit {
    pub balance: u128,
    pub active_proposals: HashSet<ProposalId>,
}