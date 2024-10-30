use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountIdValidation {
    pub allowed_accounts: Option<HashSet<AccountId>>,
    pub banned_accounts: Option<HashSet<AccountId>>, 
    pub min_account_age: Option<u64>, 

}

impl AccountIdValidation {
    pub fn validate(&self, account_id: &AccountId, account_creation_timestamp: u64, current_timestamp: u64) -> Result<(), String> {
        // Check if the account is banned
        if let Some(banned_accounts) = &self.banned_accounts {
            if banned_accounts.contains(account_id) {
                return Err(format!("Account {} is banned.", account_id));
            }
        }

        // Check if allowed accounts is defined and whether the account is in the allowed set
        if let Some(allowed_accounts) = &self.allowed_accounts {
            if !allowed_accounts.contains(account_id) {
                return Err(format!("Account {} is not in the list of allowed accounts.", account_id));
            }
        }

        // Check the minimum account age, if specified
        if let Some(min_age) = self.min_account_age {
            let account_age = current_timestamp.saturating_sub(account_creation_timestamp);
            if account_age < min_age {
                return Err(format!(
                    "Account {} does not meet the minimum age requirement of {} seconds. Account age: {} seconds.",
                    account_id, min_age, account_age
                ));
            }
        }

        // If all checks pass, return Ok
        Ok(())
    }
}
