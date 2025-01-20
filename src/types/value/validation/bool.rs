use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct BooleanValidation {
    pub must_be_true: Option<bool>, // If set, enforces that the value must be true or false
}