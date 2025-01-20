use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct MediaValidation {
    pub banned_media: HashSet<MediaHash>,         // Media that are not allowed
    pub allowed_formats: Option<HashSet<String>>, // Allowed formats (e.g., "mp4", "png", "mp3")
    pub max_duration: Option<u64>,                // Maximum duration in seconds (for media with duration)
}

impl MediaValidation {
    pub fn validate(&self, media_hash: &MediaHash, format: &str, duration: Option<u64>) -> Result<(), String> {
        // Check if the media is banned
        if self.banned_media.contains(media_hash) {
            return Err(format!("Media {} is banned.", media_hash));
        }

        // Check if the format is allowed
        if let Some(allowed_formats) = &self.allowed_formats {
            if !allowed_formats.contains(format) {
                return Err(format!("Format '{}' is not allowed.", format));
            }
        }

        // Check if the duration is within the allowed limit (if applicable)
        if let Some(max_duration) = self.max_duration {
            if let Some(duration) = duration {
                if duration > max_duration {
                    return Err(format!(
                        "Duration {} seconds exceeds the maximum allowed duration of {} seconds.",
                        duration, max_duration
                    ));
                }
            }
        }

        // If all checks pass, return Ok
        Ok(())
    }
}
