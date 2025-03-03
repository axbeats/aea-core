use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum VideoUpdate {
    Title(String),
    Description(Option<String>),
    Permissions(VideoPermissions),
}