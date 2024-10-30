use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct SingleValue {
    pub value: ValueType,
    pub validation: Option<ValidationType>,
}

impl SingleValue {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(validation) = &self.validation {
            if !self.value.matches(validation) {
                return Err("Validation does not match the ValueType".to_string());
            }
        }
        Ok(())
    }
}