use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Governed<T> {
    pub value: T,
    pub method: GovernanceMethod,
    pub reference: GovernedReference,
}

impl<T> Governed<T>
where
    T: GovernedTarget,
{
    /// Updates the governed object with the given input.
    pub fn update(&mut self, input: T::Input) -> Result<(), T::Error> {
        self.value.update_value(input)
    }
}

pub trait GovernedTarget {
    type Input;
    type Error;

    /// Updates the governed object with the given input.
    fn update_value(&mut self, input: Self::Input) -> Result<(), Self::Error>;
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct GovernedReference {
    pub primary_key: String,
    pub secondary_key: String,
    pub identifier: Option<Identifier>,
}