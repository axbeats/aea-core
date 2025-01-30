use crate::*;

pub trait ChoiceSingleInterface {
    fn update_elected_single(&mut self, id: &ValueId, new_value: String);
}

pub trait ChoiceMultipleInterface {
    fn update_elected_multiple(&mut self, id: &ValueId, new_value: Vec<String>);
}