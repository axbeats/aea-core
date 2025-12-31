//! Gas validation for DAO operations.

use near_sdk::Gas;

use super::calculator::{DaoGasCalculator, GasBreakdown};
use super::constants::*;
use crate::DaoInput;

/// Result of gas validation for a DAO deployment.
#[derive(Debug, Clone)]
pub enum GasValidationResult {
    /// Gas requirements fit within transaction limits.
    Valid(GasBreakdown),

    /// Gas exceeds transaction limit.
    ExceedsLimit {
        breakdown: GasBreakdown,
        excess: Gas,
        suggestion: String,
    },

    /// Too many token roles for a single transaction.
    TooManyTokenRoles { count: u32, max: u32 },

    /// Too many total roles for a single transaction.
    TooManyRoles { count: u32, max: u32 },
}

impl GasValidationResult {
    /// Check if the validation passed.
    pub fn is_valid(&self) -> bool {
        matches!(self, GasValidationResult::Valid(_))
    }

    /// Get the gas breakdown if available.
    pub fn breakdown(&self) -> Option<&GasBreakdown> {
        match self {
            GasValidationResult::Valid(b) => Some(b),
            GasValidationResult::ExceedsLimit { breakdown, .. } => Some(breakdown),
            _ => None,
        }
    }

    /// Get the error message for failed validation.
    pub fn error_message(&self) -> Option<String> {
        match self {
            GasValidationResult::Valid(_) => None,
            GasValidationResult::ExceedsLimit {
                excess, suggestion, ..
            } => Some(format!(
                "ERR_GAS_EXCEEDS_LIMIT: Requires {} TGas over the 300 TGas limit. {}",
                excess.as_gas() / 1_000_000_000_000,
                suggestion
            )),
            GasValidationResult::TooManyTokenRoles { count, max } => Some(format!(
                "ERR_TOO_MANY_TOKEN_ROLES: {} token roles exceeds maximum of {} per DAO. Each DAO can only have one governance token.",
                count, max
            )),
            GasValidationResult::TooManyRoles { count, max } => Some(format!(
                "ERR_TOO_MANY_ROLES: {} roles exceeds maximum of {} per transaction",
                count, max
            )),
        }
    }
}

/// Validate gas requirements for a DAO deployment.
///
/// This function checks:
/// 1. Token role count doesn't exceed limits
/// 2. Total role count doesn't exceed limits
/// 3. Total gas estimate fits within 300 TGas transaction limit
///
/// Returns a `GasValidationResult` with either the valid breakdown
/// or an error with actionable suggestions.
pub fn validate_dao_gas(input: &DaoInput) -> GasValidationResult {
    // Check token role count first (most expensive)
    let token_role_count = DaoGasCalculator::count_token_roles(&input.roles);
    if token_role_count > MAX_TOKEN_ROLES_PER_TX {
        return GasValidationResult::TooManyTokenRoles {
            count: token_role_count,
            max: MAX_TOKEN_ROLES_PER_TX,
        };
    }

    // Check total role count
    let total_roles = input.roles.len() as u32;
    if total_roles > MAX_TOTAL_ROLES_PER_TX {
        return GasValidationResult::TooManyRoles {
            count: total_roles,
            max: MAX_TOTAL_ROLES_PER_TX,
        };
    }

    // Calculate detailed breakdown
    let breakdown = DaoGasCalculator::calculate_dao_init_gas(input);

    if breakdown.fits_in_transaction() {
        GasValidationResult::Valid(breakdown)
    } else {
        let excess = breakdown.excess().unwrap();
        let suggestion = generate_suggestion(input, &breakdown);
        GasValidationResult::ExceedsLimit {
            breakdown,
            excess,
            suggestion,
        }
    }
}

/// Generate actionable suggestion for reducing gas usage.
fn generate_suggestion(input: &DaoInput, _breakdown: &GasBreakdown) -> String {
    let elected_roles = DaoGasCalculator::count_elected_roles(&input.roles);

    if elected_roles > 2 {
        return format!(
            "Reduce elected roles from {} to 2 or fewer. Elected roles require ~30 TGas each.",
            elected_roles
        );
    }

    if input.roles.len() > 5 {
        return format!(
            "Reduce total roles from {} to 5 or fewer for initial deployment. Additional roles can be added later.",
            input.roles.len()
        );
    }

    if !input.contracts.is_empty() && !input.rules.is_empty() {
        return "Consider deploying without contracts or rules initially. These can be added after DAO creation.".to_string();
    }

    "Consider reducing the number of roles, contracts, or rules in this deployment.".to_string()
}

/// Assert that gas requirements are valid, panicking with a descriptive error if not.
///
/// This is a convenience function for use in smart contracts where
/// you want to fail early with a clear error message.
pub fn assert_valid_dao_gas(input: &DaoInput) {
    let result = validate_dao_gas(input);
    if let Some(error) = result.error_message() {
        near_sdk::env::panic_str(&error);
    }
}

/// Log the gas breakdown for debugging purposes.
///
/// Useful during development to compare estimated vs actual gas usage.
#[cfg(not(target_arch = "wasm32"))]
pub fn log_gas_breakdown(breakdown: &GasBreakdown) {
    println!("Gas Breakdown:");
    println!("  DAO Deploy:  {} TGas", breakdown.dao_deploy.as_gas() / 1_000_000_000_000);
    println!("  Profile:     {} TGas", breakdown.profile.as_gas() / 1_000_000_000_000);
    println!("  Roles:       {} TGas", breakdown.roles.as_gas() / 1_000_000_000_000);
    println!("  Contracts:   {} TGas", breakdown.contracts.as_gas() / 1_000_000_000_000);
    println!("  Rules:       {} TGas", breakdown.rules.as_gas() / 1_000_000_000_000);
    println!("  Overhead:    {} TGas", breakdown.overhead.as_gas() / 1_000_000_000_000);
    println!("  ─────────────────────");
    println!("  Total:       {} TGas", breakdown.total_tgas());
    println!("  Remaining:   {} TGas", breakdown.remaining().as_gas() / 1_000_000_000_000);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ProfileInput, Policy, RoleInput, ContractInput, RuleInput};

    fn minimal_dao_input() -> DaoInput {
        DaoInput {
            profile: ProfileInput::example(),
            default_policy: Policy::example(),
            roles: vec![],
            contracts: vec![],
            rules: vec![],
        }
    }

    #[test]
    fn test_minimal_dao_valid() {
        let input = minimal_dao_input();
        let result = validate_dao_gas(&input);
        assert!(result.is_valid());
    }

    #[test]
    fn test_single_token_role_valid() {
        let mut input = minimal_dao_input();
        input.roles = vec![RoleInput::example_token()];

        let result = validate_dao_gas(&input);
        assert!(result.is_valid());

        let breakdown = result.breakdown().unwrap();
        // Should be around 195 TGas (60 base + 120 token role + 15 overhead)
        assert!(breakdown.total_tgas() < 210);
    }

    #[test]
    fn test_one_token_role_valid() {
        let mut input = minimal_dao_input();
        input.roles = vec![RoleInput::example_token()];

        let result = validate_dao_gas(&input);
        assert!(result.is_valid());
    }

    #[test]
    fn test_too_many_token_roles_rejected() {
        let mut input = minimal_dao_input();
        input.roles = vec![
            RoleInput::example_token(),
            RoleInput::example_token(), // 2nd token role - exceeds max of 1
        ];

        let result = validate_dao_gas(&input);
        assert!(!result.is_valid());
        assert!(matches!(
            result,
            GasValidationResult::TooManyTokenRoles { count: 2, max: 1 }
        ));
    }

    #[test]
    fn test_error_message_generation() {
        let result = GasValidationResult::TooManyTokenRoles { count: 2, max: 1 };
        let message = result.error_message().unwrap();
        assert!(message.contains("ERR_TOO_MANY_TOKEN_ROLES"));
        assert!(message.contains("2"));
        assert!(message.contains("1"));
    }
}
