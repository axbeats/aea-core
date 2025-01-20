use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct BytesValidation {
    pub allowed_file_types: Option<HashSet<String>>, // E.g., "png", "jpg"
    pub max_size: Option<u64>,                       // Maximum size in bytes
}

impl BytesValidation {
    pub fn validate(&self, file_type: &str, file_size: u64) -> Result<(), String> {
        // Check if the file type is allowed
        if let Some(allowed_file_types) = &self.allowed_file_types {
            if !allowed_file_types.contains(file_type) {
                return Err(format!("File type '{}' is not allowed.", file_type));
            }
        }

        // Check if the file size is within the allowed limit
        if let Some(max_size) = self.max_size {
            if file_size > max_size {
                return Err(format!(
                    "File size {} bytes exceeds the maximum allowed size of {} bytes.",
                    file_size, max_size
                ));
            }
        }

        // If all checks pass, return Ok
        Ok(())
    }
}
