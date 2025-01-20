use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct SetValue {
    pub values: HashSet<ValueType>,
    pub validation: Option<ValidationType>,
}

impl SetValue {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(first_value) = self.values.iter().next() {
            // Check if all values have the same type
            let first_type = std::mem::discriminant(first_value);
            if !self.values.iter().all(|v| std::mem::discriminant(v) == first_type) {
                return Err("Set contains mixed ValueType variants".to_string());
            }

            // Check if validation matches the value type
            if let Some(validation) = &self.validation {
                if !first_value.matches(validation) {
                    return Err("Validation does not match the ValueType".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn is_same_size(&self, new_values: &HashSet<ValueType>) -> bool {
        self.values.len() == new_values.len()
    }

}