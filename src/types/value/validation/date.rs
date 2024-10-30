use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct DateValidation {
    pub min_date: Option<u64>, // Earliest allowed date (timestamp)
    pub max_date: Option<u64>, // Latest allowed date (timestamp)
}

impl DateValidation {
    pub fn validate(&self, date: u64) -> Result<(), String> {
        // Check minimum date
        if let Some(min_date) = self.min_date {
            if date < min_date {
                return Err(format!("Date {} is earlier than the minimum allowed date {}.", date, min_date));
            }
        }

        // Check maximum date
        if let Some(max_date) = self.max_date {
            if date > max_date {
                return Err(format!("Date {} is later than the maximum allowed date {}.", date, max_date));
            }
        }

        // If all checks pass, return Ok
        Ok(())
    }
}
