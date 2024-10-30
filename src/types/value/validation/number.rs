use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct NumberValidation<T> {
    pub range: Option<(T, T)>,     // Minimum and maximum allowed values
    pub required_sum: Option<T>,   // The exact sum that the numbers must add up to
}

use std::ops::Add;

impl<T> NumberValidation<T>
where
    T: PartialOrd + Add<Output = T> + Copy + Default + std::fmt::Display,
{
    pub fn validate(&self, numbers: &[T]) -> Result<(), String> {
        // Validate each number against the range
        if let Some((min, max)) = self.range {
            for &number in numbers {
                if number < min || number > max {
                    return Err(format!(
                        "Number {} is out of the allowed range: {} - {}.",
                        number, min, max
                    ));
                }
            }
        }

        // Validate the sum of the numbers
        if let Some(required_sum) = self.required_sum {
            let sum: T = numbers.iter().copied().fold(T::default(), Add::add);
            if sum != required_sum {
                return Err(format!(
                    "The sum of the numbers is {} but the required sum is {}.",
                    sum, required_sum
                ));
            }
        }

        // If all checks pass, return Ok
        Ok(())
    }
}

