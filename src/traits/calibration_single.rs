use crate::*;

// pub trait CalibrationSingleInterface {
//     type Object: CalibrationSingleObject;

//     fn get_object(&self, identifier: CalibrationIdentifier) -> &Self::Object;
//     fn get_object_mut(&mut self, identifier: CalibrationIdentifier) -> &mut Self::Object;
//     fn calibrate_single(
//         &mut self,
//         identifier: CalibrationIdentifier,
//         single_vote: SingleVote,
//         adjustment_factor: AdjustmentFactor,
//     ) -> Result<(), CalibrationError>;
// }

// pub trait CalibrationSingleObject {
//     type Item: CalibrationItem;

//     /// Retrieves an item by name (for collection types).
//     fn get_item(&self, name: &String) -> Option<Self::Item>;

//     /// Sets an item in the calibration object (for collection types).
//     fn set_item(&mut self, name: &String, item: Self::Item);

//     /// Retrieves the single item (for scalar types).
//     fn get_value(&self) -> Option<Self::Item>;

//     /// Sets the single item (for scalar types).
//     fn set_value(&mut self, item: Self::Item);

//     /// Performs calibration on the object.
//     fn calibrate_single(
//         &mut self,
//         single_vote: SingleVote,
//         adjustment_factor: AdjustmentFactor,
//     ) -> Result<(), CalibrationError> {
//         let calibration_id: AccountId = CALIBRATION_ID.parse().expect("ERR_CALIBRATION_ID");
//         if env::predecessor_account_id() != calibration_id {
//             return Err(CalibrationError::Unauthorized);
//         }
    
//         // Extract item based on whether `id` is empty
//         let mut item = if !single_vote.id.is_empty() {
//             // For collection types
//             self.get_item(&single_vote.id)
//                 .ok_or_else(|| CalibrationError::NotFound(single_vote.id.clone()))?
//         } else {
//             // For scalar types
//             self.get_value()
//                 .ok_or_else(|| CalibrationError::NotFound("Value".to_string()))?
//         };
    
//         // Calculate the adjustment amount
//         let adjustment = match adjustment_factor {
//             AdjustmentFactor::Percentage(percentage) => {
//                 (item.get_percentage() * percentage as u128) / 100
//             }
//             AdjustmentFactor::Fixed(amount) => amount,
//         };
    
//         // Adjust the item's percentage
//         let new_percentage = match single_vote.direction {
//             AdjustmentDirection::Increase => item
//                 .get_percentage()
//                 .checked_add(adjustment)
//                 .ok_or_else(|| CalibrationError::Overflow(single_vote.id.clone()))?,
//             AdjustmentDirection::Decrease => item
//                 .get_percentage()
//                 .checked_sub(adjustment)
//                 .ok_or_else(|| CalibrationError::Underflow(single_vote.id.clone()))?,
//         };
    
//         item.set_percentage(new_percentage);
    
//         // Set the updated item back
//         if !single_vote.id.is_empty() {
//             self.set_item(&single_vote.id, item);
//         } else {
//             self.set_value(item);
//         }
    
//         Ok(())
//     }
    

//     /// Retrieves all items in the calibration object (default empty for scalar types).
//     fn items(&self) -> Vec<Self::Item> {
//         Vec::new()
//     }
// }

