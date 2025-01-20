use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub enum ValueStructure {
    Single(SingleValue),
    Set(SetValue),
    Map(MapValue),
}

impl ValueStructure {
    pub fn is_same_type(&self, other: &ValueStructure) -> bool {
        match (self, other) {
            (ValueStructure::Single(old_single), ValueStructure::Single(new_single)) => {
                // Compare the types of the single values
                std::mem::discriminant(&old_single.value) == std::mem::discriminant(&new_single.value)
            }
            (ValueStructure::Set(old_set), ValueStructure::Set(new_set)) => {
                // Check if both sets are empty or if all elements in the sets have the same type
                if old_set.values.is_empty() && new_set.values.is_empty() {
                    true
                } else if let (Some(old_first), Some(new_first)) = (
                    old_set.values.iter().next(),
                    new_set.values.iter().next(),
                ) {
                    std::mem::discriminant(old_first) == std::mem::discriminant(new_first)
                } else {
                    false
                }
            }
            (ValueStructure::Map(old_map), ValueStructure::Map(new_map)) => {
                // Check if both maps are empty or if all values in the maps have the same type
                if old_map.values.is_empty() && new_map.values.is_empty() {
                    true
                } else if let (Some((_, old_first)), Some((_, new_first))) = (
                    old_map.values.iter().next(),
                    new_map.values.iter().next(),
                ) {
                    std::mem::discriminant(old_first) == std::mem::discriminant(new_first)
                } else {
                    false
                }
            }
            // If they are different variants (e.g., Single vs Set), return false
            _ => false,
        }
    }
}
