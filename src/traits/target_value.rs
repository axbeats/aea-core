use crate::*;

pub trait TargetValue {
    fn create_value(&mut self, id: ValueId, method_input: VoteMethodInput) -> VerifyValueResult;
}

// pub trait TargetValue {
//     fn verify_value(&mut self, id: GovernedValueId, method_input: VoteMethodInput) -> VerifyValueResult;
//     fn get_value(&self, id: GovernedValueId) -> Option<VerifyValueResult>;
//     fn update_value(&mut self, id: GovernedValueId, new_value: VoteMethodInput) -> bool;
// }
