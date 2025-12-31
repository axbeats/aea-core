//! Gas and deposit calculation utilities for DAO operations.

use near_sdk::{Gas, NearToken};

use super::constants::*;
use crate::{DaoInput, RoleInput, RoleKindInput, ContractInput, RuleInput};

/// Detailed breakdown of gas requirements for DAO initialization.
/// All values are in Gas units (not TGas).
#[derive(Debug, Clone)]
pub struct GasBreakdown {
    /// Gas for DAO contract deployment
    pub dao_deploy: Gas,
    /// Gas for profile creation
    pub profile: Gas,
    /// Gas for all role initializations
    pub roles: Gas,
    /// Gas for contract registrations
    pub contracts: Gas,
    /// Gas for rule initializations
    pub rules: Gas,
    /// Reserved gas for callbacks and overhead
    pub overhead: Gas,
    /// Total gas required
    pub total: Gas,
}

impl GasBreakdown {
    /// Check if this breakdown fits within transaction limits
    pub fn fits_in_transaction(&self) -> bool {
        self.total.as_gas() <= MAX_TRANSACTION_GAS.as_gas()
    }

    /// Get excess gas if over the limit, None if within limits
    pub fn excess(&self) -> Option<Gas> {
        if self.total.as_gas() > MAX_TRANSACTION_GAS.as_gas() {
            Some(Gas::from_gas(self.total.as_gas() - MAX_TRANSACTION_GAS.as_gas()))
        } else {
            None
        }
    }

    /// Get remaining gas available within transaction limits
    pub fn remaining(&self) -> Gas {
        if self.total.as_gas() >= MAX_TRANSACTION_GAS.as_gas() {
            Gas::from_gas(0)
        } else {
            Gas::from_gas(MAX_TRANSACTION_GAS.as_gas() - self.total.as_gas())
        }
    }

    /// Convert total to TGas for display
    pub fn total_tgas(&self) -> u64 {
        self.total.as_gas() / 1_000_000_000_000
    }
}

/// Calculator for estimating gas requirements for DAO operations.
pub struct DaoGasCalculator;

impl DaoGasCalculator {
    /// Calculate total gas needed for a complete DAO deployment and initialization.
    ///
    /// This includes:
    /// - DAO contract deployment
    /// - Profile creation
    /// - All role initializations (token, regular, elected)
    /// - Contract registrations
    /// - Rule initializations
    /// - Callback overhead
    pub fn calculate_dao_init_gas(input: &DaoInput) -> GasBreakdown {
        let dao_deploy = DAO_DEPLOY_GAS;
        let profile = PROFILE_CREATE_GAS;
        let roles = Self::calculate_roles_gas(&input.roles);
        let contracts = Self::calculate_contracts_gas(&input.contracts);
        let rules = Self::calculate_rules_gas(&input.rules);
        let overhead = Self::calculate_overhead_gas(input);

        let total = dao_deploy
            .saturating_add(profile)
            .saturating_add(roles)
            .saturating_add(contracts)
            .saturating_add(rules)
            .saturating_add(overhead);

        GasBreakdown {
            dao_deploy,
            profile,
            roles,
            contracts,
            rules,
            overhead,
            total,
        }
    }

    /// Calculate gas for all role initializations.
    pub fn calculate_roles_gas(roles: &[RoleInput]) -> Gas {
        let mut total = Gas::from_gas(0);

        for role in roles {
            let role_gas = Self::gas_for_role_kind(&role.kind);
            total = total.saturating_add(role_gas);
        }

        total
    }

    /// Get gas required for a specific role kind.
    pub fn gas_for_role_kind(kind: &RoleKindInput) -> Gas {
        match kind {
            RoleKindInput::Token(_) => TOKEN_ROLE_INIT_GAS,
            RoleKindInput::Elected(_) => ELECTED_ROLE_INIT_GAS,
            RoleKindInput::Followers
            | RoleKindInput::Subscribers
            | RoleKindInput::Region(_)
            | RoleKindInput::Agent(_) => REGULAR_ROLE_INIT_GAS,
        }
    }

    /// Calculate gas for contract registrations.
    pub fn calculate_contracts_gas(contracts: &[ContractInput]) -> Gas {
        if contracts.is_empty() {
            return Gas::from_gas(0);
        }

        // Base registration cost + per-contract overhead
        CONTRACT_REGISTER_GAS.saturating_add(
            PER_CONTRACT_GAS.saturating_mul(contracts.len() as u64),
        )
    }

    /// Calculate gas for rule initializations.
    pub fn calculate_rules_gas(rules: &[RuleInput]) -> Gas {
        if rules.is_empty() {
            return Gas::from_gas(0);
        }

        // Base initialization cost + per-rule overhead
        RULE_INIT_GAS.saturating_add(
            PER_RULE_GAS.saturating_mul(rules.len() as u64),
        )
    }

    /// Calculate overhead gas for callback chains.
    fn calculate_overhead_gas(input: &DaoInput) -> Gas {
        // Base overhead for the main callback chain
        let mut overhead = CALLBACK_GAS_CHAINED;

        // Additional overhead based on number of phases with content
        if !input.roles.is_empty() {
            overhead = overhead.saturating_add(CALLBACK_GAS_STORAGE);
        }
        if !input.contracts.is_empty() {
            overhead = overhead.saturating_add(CALLBACK_GAS_STORAGE);
        }
        if !input.rules.is_empty() {
            overhead = overhead.saturating_add(CALLBACK_GAS_STORAGE);
        }

        overhead
    }

    /// Count token roles in input.
    pub fn count_token_roles(roles: &[RoleInput]) -> u32 {
        roles
            .iter()
            .filter(|r| matches!(r.kind, RoleKindInput::Token(_)))
            .count() as u32
    }

    /// Count elected roles in input.
    pub fn count_elected_roles(roles: &[RoleInput]) -> u32 {
        roles
            .iter()
            .filter(|r| matches!(r.kind, RoleKindInput::Elected(_)))
            .count() as u32
    }

    /// Count regular (non-token, non-elected) roles.
    pub fn count_regular_roles(roles: &[RoleInput]) -> u32 {
        roles
            .iter()
            .filter(|r| {
                !matches!(r.kind, RoleKindInput::Token(_))
                    && !matches!(r.kind, RoleKindInput::Elected(_))
            })
            .count() as u32
    }

    /// Calculate callback gas for role initialization based on remaining roles.
    /// This replaces the unbounded `CALLBACK_GAS * 3 * remaining_roles` formula.
    pub fn calculate_role_callback_gas(remaining_roles: usize, roles: &[RoleInput]) -> Gas {
        // Cap the multiplier to prevent unbounded gas allocation
        let capped_remaining = remaining_roles.min(MAX_TOTAL_ROLES_PER_TX as usize);

        // Base callback gas
        let mut callback_gas = CALLBACK_GAS_CHAINED;

        // Add gas for remaining role processing (limited)
        for i in 0..capped_remaining {
            if let Some(role) = roles.get(i) {
                let role_overhead = match &role.kind {
                    RoleKindInput::Token(_) => CALLBACK_GAS_STORAGE,
                    _ => CALLBACK_GAS_SIMPLE,
                };
                callback_gas = callback_gas.saturating_add(role_overhead);
            }
        }

        callback_gas
    }
}

// ============================================================================
// Deposit Calculations
// ============================================================================

/// Detailed breakdown of deposit requirements for DAO initialization.
/// All values are in yoctoNEAR (10^-24 NEAR).
#[derive(Debug, Clone)]
pub struct DepositBreakdown {
    /// Deposit for DAO account creation
    pub dao_account: NearToken,
    /// Deposit for token account (if DAO has a token role)
    pub token_account: NearToken,
    /// Deposit for staking account (if DAO has a token role)
    pub staking_account: NearToken,
    /// Whether this DAO has a token role
    pub has_token_role: bool,
    /// Total deposit required
    pub total: NearToken,
}

impl DepositBreakdown {
    /// Convert total to NEAR for display
    pub fn total_near(&self) -> f64 {
        self.total.as_yoctonear() as f64 / 1e24
    }

    /// Get the deposit needed for roles only (to pass to role contract)
    pub fn roles_deposit(&self) -> NearToken {
        NearToken::from_yoctonear(
            self.token_account.as_yoctonear() + self.staking_account.as_yoctonear()
        )
    }
}

/// Calculator for estimating deposit requirements for DAO operations.
pub struct DaoDepositCalculator;

impl DaoDepositCalculator {
    /// Calculate total deposit needed for a complete DAO deployment and initialization.
    ///
    /// This includes:
    /// - DAO account creation (always 0.1 NEAR)
    /// - Token account (0.1 NEAR if has token role)
    /// - Staking account (0.1 NEAR if has token role)
    ///
    /// Total: 0.1 NEAR (no token) or 0.3 NEAR (with token)
    pub fn calculate_dao_deposit(input: &DaoInput) -> DepositBreakdown {
        let dao_account = MIN_DAO_DEPOSIT;
        let has_token_role = DaoGasCalculator::count_token_roles(&input.roles) > 0;

        // Single token role creates: 1 token account + 1 staking account
        let (token_account, staking_account) = if has_token_role {
            (MIN_TOKEN_DEPOSIT, MIN_STAKING_DEPOSIT)
        } else {
            (NearToken::from_yoctonear(0), NearToken::from_yoctonear(0))
        };

        let total = NearToken::from_yoctonear(
            dao_account.as_yoctonear()
            + token_account.as_yoctonear()
            + staking_account.as_yoctonear()
        );

        DepositBreakdown {
            dao_account,
            token_account,
            staking_account,
            has_token_role,
            total,
        }
    }

    /// Calculate deposit needed for role initialization only.
    /// This is the amount to pass to the role contract.
    /// Returns TOKEN_ROLE_DEPOSIT (0.2 NEAR) if has token role, 0 otherwise.
    pub fn calculate_roles_deposit(roles: &[RoleInput]) -> NearToken {
        let has_token_role = DaoGasCalculator::count_token_roles(roles) > 0;
        if has_token_role {
            TOKEN_ROLE_DEPOSIT
        } else {
            NearToken::from_yoctonear(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gas_breakdown_fits_in_transaction() {
        let breakdown = GasBreakdown {
            dao_deploy: Gas::from_tgas(45),
            profile: Gas::from_tgas(15),
            roles: Gas::from_tgas(70),
            contracts: Gas::from_tgas(0),
            rules: Gas::from_tgas(0),
            overhead: Gas::from_tgas(15),
            total: Gas::from_tgas(145),
        };

        assert!(breakdown.fits_in_transaction());
        assert!(breakdown.excess().is_none());
    }

    #[test]
    fn test_gas_breakdown_exceeds_limit() {
        let breakdown = GasBreakdown {
            dao_deploy: Gas::from_tgas(45),
            profile: Gas::from_tgas(15),
            roles: Gas::from_tgas(250),
            contracts: Gas::from_tgas(0),
            rules: Gas::from_tgas(0),
            overhead: Gas::from_tgas(15),
            total: Gas::from_tgas(325),
        };

        assert!(!breakdown.fits_in_transaction());
        assert!(breakdown.excess().is_some());
        assert_eq!(breakdown.excess().unwrap().as_gas(), Gas::from_tgas(25).as_gas());
    }
}
