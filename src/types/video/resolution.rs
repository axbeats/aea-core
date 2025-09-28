use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct VideoResolution {
    pub width: u64,
    pub height: u64,
}

impl Default for VideoResolution {
    fn default() -> Self {
        Self { width: 1080, height: 1920 }
    }
}