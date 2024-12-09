use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Governed<T> {
    pub value: T,
    pub primary_key: String,
    pub secondary_key: String,
    pub identifier: String,
    pub governance_method: VoteMethod,
}


impl<T> Governed<T>
where
    T: GovernedObject,
{
    /// Updates the governed object with the given input.
    pub fn update(&mut self, input: T::Input) -> Result<(), T::Error> {
        self.value.update_value(input)
    }
}

pub trait GovernedObject {
    type Input;
    type Error;

    /// Updates the governed object with the given input.
    fn update_value(&mut self, input: Self::Input) -> Result<(), Self::Error>;
}
