use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct StringValidation {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub allowed_chars: Option<HashSet<String>>,
    pub disallowed_chars: Option<HashSet<String>>,
    pub disallowed_words: Option<HashSet<String>>,
    pub case_sensitive: Option<bool>, // Whether validation is case-sensitive
    pub required_prefix: Option<String>,
    pub required_suffix: Option<String>,
}

impl StringValidation {
    pub fn validate(&self, input: &str) -> Result<(), String> {
        let input_to_validate = match self.case_sensitive {
            Some(false) => input.to_lowercase(),
            _ => input.to_string(),
        };

        // Check minimum length
        if let Some(min_length) = self.min_length {
            if input.len() < min_length {
                return Err(format!("String is too short. Minimum length is {}", min_length));
            }
        }

        // Check maximum length
        if let Some(max_length) = self.max_length {
            if input.len() > max_length {
                return Err(format!("String is too long. Maximum length is {}", max_length));
            }
        }

        // Check required prefix
        if let Some(prefix) = &self.required_prefix {
            if !input_to_validate.starts_with(prefix) {
                return Err(format!("String must start with the prefix '{}'", prefix));
            }
        }

        // Check required suffix
        if let Some(suffix) = &self.required_suffix {
            if !input_to_validate.ends_with(suffix) {
                return Err(format!("String must end with the suffix '{}'", suffix));
            }
        }

        // Check for allowed characters
        if let Some(allowed_chars) = &self.allowed_chars {
            for c in input_to_validate.chars() {
                if !allowed_chars.contains(&c.to_string()) {
                    return Err(format!("Character '{}' is not allowed.", c));
                }
            }
        }

        // Check for disallowed characters
        if let Some(disallowed_chars) = &self.disallowed_chars {
            for c in input_to_validate.chars() {
                if disallowed_chars.contains(&c.to_string()) {
                    return Err(format!("Character '{}' is disallowed.", c));
                }
            }
        }

        // Check for disallowed words
        if let Some(disallowed_words) = &self.disallowed_words {
            for word in disallowed_words {
                if input_to_validate.contains(word) {
                    return Err(format!("The word '{}' is disallowed.", word));
                }
            }
        }

        // If all checks pass, return Ok
        Ok(())
    }
}
