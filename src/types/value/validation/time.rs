use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct TimeValidation {
    pub min_timestamp: Option<u64>, // Earliest allowed date (timestamp)
    pub max_timestamp: Option<u64>, // Latest allowed date (timestamp)
}

impl TimeValidation {
    pub fn validate(&self, timestamp: u64) -> Result<(), String> {
        // Check minimum timestamp
        if let Some(min_timestamp) = self.min_timestamp {
            if timestamp < min_timestamp {
                return Err(format!(
                    "Timestamp {} is earlier than the minimum allowed timestamp {}.",
                    timestamp, min_timestamp
                ));
            }
        }

        // Check maximum timestamp
        if let Some(max_timestamp) = self.max_timestamp {
            if timestamp > max_timestamp {
                return Err(format!(
                    "Timestamp {} is later than the maximum allowed timestamp {}.",
                    timestamp, max_timestamp
                ));
            }
        }

        // If all checks pass, return Ok
        Ok(())
    }
}
