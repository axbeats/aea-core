//! Centralized gas constants for NEAR smart contract operations.
//!
//! All gas values are conservative estimates based on measurements plus a 15% buffer.
//! These constants should be calibrated after testnet deployment by measuring actual gas usage.

use near_sdk::Gas;

// ============================================================================
// Base Gas Units
// ============================================================================

/// 1 TGas = 10^12 gas units (convenience constant)
pub const TGAS: Gas = Gas::from_tgas(1);

/// Maximum gas per NEAR transaction (hard protocol limit)
pub const MAX_TRANSACTION_GAS: Gas = Gas::from_tgas(300);

/// Minimum gas reserved for final callback/cleanup operations
pub const RESERVED_GAS: Gas = Gas::from_tgas(10);

/// Gas available for actual operations (300 - 10 reserved)
pub const USABLE_GAS: Gas = Gas::from_tgas(290);

// ============================================================================
// Cross-Contract Callback Constants
// ============================================================================

/// Standard gas for simple callbacks (state updates, logging, event emission)
/// Use for callbacks that only update local state and emit events.
pub const CALLBACK_GAS_SIMPLE: Gas = Gas::from_tgas(5);

/// Gas for callbacks with storage operations (inserts, lookups, iterations)
/// Use for callbacks that read/write to contract storage.
pub const CALLBACK_GAS_STORAGE: Gas = Gas::from_tgas(10);

/// Gas for callbacks that spawn additional cross-contract promises
/// Use for callbacks that chain into more external calls.
pub const CALLBACK_GAS_CHAINED: Gas = Gas::from_tgas(15);

/// Legacy constant for backward compatibility (equals CALLBACK_GAS_SIMPLE)
/// New code should use the specific constants above.
pub const CALLBACK_GAS: Gas = CALLBACK_GAS_SIMPLE;

// ============================================================================
// Contract Deployment Gas Constants (using Global Contracts)
// ============================================================================

/// Gas for DAO contract deployment via use_global_contract_by_account_id
/// Actual measured usage: ~5-6 TGas for create_account + transfer + use_global + new()
/// Using 15 TGas to provide buffer for complex DAO metadata
/// (Reduced from 45 TGas to leave more gas for initialization chain)
pub const DAO_DEPLOY_GAS: Gas = Gas::from_tgas(15);

/// Gas for token (ft-stream) contract deployment via use_global_contract
/// Breakdown: create_account(5) + transfer(1) + use_global(5) + new(10) + callback(5) + buffer
/// Increased from 25 to 40 to ensure enough gas for full deployment chain
pub const TOKEN_DEPLOY_GAS: Gas = Gas::from_tgas(40);

/// Gas for staking contract deployment via use_global_contract
/// Breakdown: create_account(5) + transfer(1) + use_global(5) + new(10) + callback(10) + buffer
/// Increased from 20 to 40 to ensure enough gas for full deployment chain
pub const STAKING_DEPLOY_GAS: Gas = Gas::from_tgas(40);

// ============================================================================
// Phase-Specific Gas Constants
// ============================================================================

/// Gas for profile creation (cross-contract call to profile contract + callback)
pub const PROFILE_CREATE_GAS: Gas = Gas::from_tgas(15);

/// Gas for video creation per entity (role, contract, rule)
pub const VIDEO_CREATE_GAS: Gas = Gas::from_tgas(10);

/// Gas for choice creation (elected roles)
pub const CHOICE_CREATE_GAS: Gas = Gas::from_tgas(10);

/// Gas for contract registration batch operation
pub const CONTRACT_REGISTER_GAS: Gas = Gas::from_tgas(15);

/// Gas for rule initialization batch operation
pub const RULE_INIT_GAS: Gas = Gas::from_tgas(15);

// ============================================================================
// Role-Specific Gas Constants
// ============================================================================

/// Gas for initializing a token role (most expensive role type):
/// token_deploy(25) + callback_with_staking(60) + staking_callback_with_video(30) = 115+ TGas
/// Added buffer for safety: 120 TGas
pub const TOKEN_ROLE_INIT_GAS: Gas = Gas::from_tgas(120);

/// Gas for initializing a regular role (followers, subscribers, region, agent):
/// video_create(10) + register_role(5) + callback(5) = 20 TGas
pub const REGULAR_ROLE_INIT_GAS: Gas = Gas::from_tgas(20);

/// Gas for initializing an elected role (includes choice creation):
/// video_create(10) + choice_create(10) + register_role(5) + callbacks(15) = 40 TGas
pub const ELECTED_ROLE_INIT_GAS: Gas = Gas::from_tgas(40);

// ============================================================================
// Role Limits (based on gas constraints)
// ============================================================================

/// Maximum number of token roles per DAO (enforced by design)
/// DAOs are limited to one governance token for simplicity
pub const MAX_TOKEN_ROLES_PER_TX: u32 = 1;

/// Maximum number of regular roles in a single transaction
/// Calculation: (290 usable - 60 base overhead) / 20 per role = ~11.5
pub const MAX_REGULAR_ROLES_PER_TX: u32 = 10;

/// Maximum total roles (mixed) - conservative estimate
pub const MAX_TOTAL_ROLES_PER_TX: u32 = 8;

// ============================================================================
// Base Overhead Constants
// ============================================================================

/// Base gas overhead for DAO initialization (deploy + profile)
/// DAO deploy(45) + profile(15) = 60 TGas
pub const DAO_BASE_OVERHEAD_GAS: Gas = Gas::from_tgas(60);

/// Per-contract overhead for contract registration
pub const PER_CONTRACT_GAS: Gas = Gas::from_tgas(10);

/// Per-rule overhead for rule initialization
pub const PER_RULE_GAS: Gas = Gas::from_tgas(10);

/// Gas for the entire DAO initialization chain after deployment
/// This covers: profile + roles + contracts + rules + all callbacks
/// Must be explicitly set on on_deploy_dao callback, otherwise it gets minimal default gas
///
/// Gas budget breakdown (300 TGas total):
///   - Function execution overhead: ~10 TGas (validation, logging, serialization)
///   - DAO deploy actions: ~5 TGas (create_account, transfer, use_global)
///   - DAO new() call: 15 TGas (DAO_DEPLOY_GAS - reduced from 45 based on measurements)
///   - Available for callback: 300 - 10 - 5 - 15 = 270 TGas
///
/// Callback chain needs (profile + roles + contracts + rules):
///   - profile(15) + on_profile_created(10) = 25 TGas
///   - role_init(~180 for 3 roles with token) + on_roles_initialized(5) = 185 TGas
///   - contracts(15) + on_contracts_initialized(5) = 20 TGas
///   - rules(15) + on_rules_initialized(5) + finalize(5) = 25 TGas
/// Approximate total needed: ~255 TGas
///
/// Setting to 270 TGas to use available budget.
pub const INIT_CHAIN_GAS: Gas = Gas::from_tgas(270);

// ============================================================================
// Deposit Constants (NEAR tokens for account creation and storage)
// ============================================================================

use near_sdk::NearToken;

/// Minimum deposit for DAO account creation (account + state storage)
pub const MIN_DAO_DEPOSIT: NearToken = NearToken::from_millinear(100); // 0.1 NEAR

/// Minimum deposit for token account creation (account + state storage)
/// With global contracts, this covers account creation and initial state only
pub const MIN_TOKEN_DEPOSIT: NearToken = NearToken::from_millinear(100); // 0.1 NEAR

/// Minimum deposit for staking account creation (account + state storage)
pub const MIN_STAKING_DEPOSIT: NearToken = NearToken::from_millinear(100); // 0.1 NEAR

/// Deposit per token role (token + staking accounts)
/// MIN_TOKEN_DEPOSIT + MIN_STAKING_DEPOSIT = 0.2 NEAR
pub const TOKEN_ROLE_DEPOSIT: NearToken = NearToken::from_millinear(200); // 0.2 NEAR

/// No deposit constant for convenience
pub const NO_DEPOSIT: NearToken = NearToken::from_near(0);
