use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub enum ChoiceSize {
    Single,
    Multiple(u8), // Number of elected items
}

impl ChoiceSize {
    /// Verifies that a choice size update is valid
    /// - Single can only be updated to Single (no change)
    /// - Multiple can only be updated to Multiple, but the number can change
    pub fn verify_update_choice_size(&self, updated_size: &ChoiceSize) {
        match (self, updated_size) {
            // Single can only be updated to Single
            (ChoiceSize::Single, ChoiceSize::Single) => { /* Valid update */ }
            (ChoiceSize::Single, ChoiceSize::Multiple(_)) => {
                panic!("ERR_CANNOT_CHANGE_CHOICE_SIZE_TYPE_FROM_SINGLE_TO_MULTIPLE");
            }

            // Multiple can only be updated to Multiple, but the number can change
            (ChoiceSize::Multiple(_), ChoiceSize::Multiple(_)) => { /* Valid update */ }
            (ChoiceSize::Multiple(_), ChoiceSize::Single) => {
                panic!("ERR_CANNOT_CHANGE_CHOICE_SIZE_TYPE_FROM_MULTIPLE_TO_SINGLE");
            }
        }
    }
}
