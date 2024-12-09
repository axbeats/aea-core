// use crate::*;

// // Define the ChoiceSingleInterface trait
// pub trait ChoiceSingleInterface {
//     type Object: ChoiceSingleObject;

//     /// Retrieves a ChoiceSingleObject by its identifier.
//     fn get_object(&self, identifier: ChoiceId) -> &Self::Object;

//     /// Retrieves a mutable reference to a ChoiceSingleObject by its identifier.
//     fn get_object_mut(&mut self, identifier: ChoiceId) -> &mut Self::Object;

//     /// Updates the votes for a ChoiceSingleObject identified by `identifier`.
//     fn update_votes(
//         &mut self,
//         identifier: ChoiceId,
//         choice_vote: ChoiceVote,
//     ) -> Result<(), ChoiceError> {
//         let object = self.get_object_mut(identifier);
//         object.update_votes(choice_vote)
//     }
// }


// // Define the ChoiceSingleObject trait
// pub trait ChoiceSingleObject {
//     type Item: ChoiceItem;

//     /// Retrieves a candidate by identifier.
//     fn get_candidate(&self, id: &u64) -> Option<Self::Item>;

//     /// Sets a candidate in the choice object.
//     fn set_candidate(&mut self, id: &u64, item: Self::Item);

//     /// Retrieves the currently elected item.
//     fn get_elected(&self) -> Option<Self::Item>;

//     /// Sets the elected item.
//     fn set_elected(&mut self, item: Option<Self::Item>);

//     /// Retrieves all candidates in the choice object.
//     fn candidates(&self) -> Vec<Self::Item>;

//     /// Retrieves the maximum number of options a user can vote for.
//     fn max_vote_options(&self) -> u8;

//     /// Retrieves the previous vote of a user, if any.
//     fn get_previous_vote(&self, account_id: &AccountId) -> Option<ChoiceVote>;

//     /// Sets the vote for a user.
//     fn set_vote(&mut self, account_id: &AccountId, vote: ChoiceVote);

//     /// Performs the vote update operation, handling multiple options.
//     fn update_votes(&mut self, choice_vote: ChoiceVote) -> Result<(), ChoiceError> {
//         // Ensure the number of voted options does not exceed max_vote_options
//         let max_options = self.max_vote_options() as usize;
//         if choice_vote.voted_options.len() > max_options {
//             return Err(ChoiceError::InvalidVote(format!(
//                 "Cannot vote for more than {} options",
//                 max_options
//             )));
//         }

//         let account_id = choice_vote.account_id.clone();

//         // Retrieve and remove previous vote if exists
//         if let Some(previous_vote) = self.get_previous_vote(&account_id) {
//             // Subtract previous vote weights
//             for option_id in previous_vote.voted_options {
//                 let mut candidate = self
//                     .get_candidate(&option_id)
//                     .ok_or_else(|| ChoiceError::CandidateNotFound(option_id.to_string()))?;

//                 let new_vote_count = candidate
//                     .get_vote_count()
//                     .checked_sub(previous_vote.weight)
//                     .ok_or_else(|| ChoiceError::Underflow(option_id.to_string()))?;
//                 candidate.set_vote_count(new_vote_count);

//                 self.set_candidate(&option_id, candidate);
//             }
//         }

//         // Add new vote weights
//         for option_id in &choice_vote.voted_options {
//             let mut candidate = self
//                 .get_candidate(option_id)
//                 .ok_or_else(|| ChoiceError::CandidateNotFound(option_id.to_string()))?;

//             let new_vote_count = candidate
//                 .get_vote_count()
//                 .checked_add(choice_vote.weight)
//                 .ok_or_else(|| ChoiceError::Overflow(option_id.to_string()))?;
//             candidate.set_vote_count(new_vote_count);

//             self.set_candidate(option_id, candidate);
//         }

//         // Store the new vote
//         self.set_vote(&account_id, choice_vote);

//         // Recalculate the elected item
//         let elected_candidate = self
//             .candidates()
//             .into_iter()
//             .max_by_key(|item| item.get_vote_count());

//         // Update the elected item
//         self.set_elected(elected_candidate);

//         Ok(())
//     }
// }

// // // Define the ChoiceSingleItem trait
// // pub trait ChoiceItem {
// //     /// Gets the vote count or score for the item.
// //     fn get_vote_count(&self) -> u128;

// //     /// Sets the vote count or score for the item.
// //     fn set_vote_count(&mut self, count: u128);

// //     /// Gets the identifier of the item (e.g., option ID).
// //     fn get_identifier(&self) -> String;
// // }


// // #[derive(Debug)]
// // pub enum ChoiceError {
// //     InvalidVote(String),
// //     CandidateNotFound(String),
// //     Overflow(String),
// //     Underflow(String),
// //     Unauthorized,
// //     // Add other relevant error variants
// // }
