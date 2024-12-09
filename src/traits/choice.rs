use crate::*;

pub trait ChoiceInterface<Object: ChoiceObject> {
    /// Retrieves a `ChoiceObject` by its identifier.
    fn get_object(&self, identifier: ChoiceIdentifier) -> Option<Object>;

    /// Sets the `ChoiceObject` for a given identifier.
    fn set_object(&mut self, identifier: ChoiceIdentifier, object: Object) -> Result<(), ChoiceError>;

    /// Updates the elected candidates for a `ChoiceObject` identified by `identifier`.
    fn update_elected(
        &mut self,
        choice_id: ChoiceId,
        identifier: ChoiceIdentifier,
        elected_candidates: Vec<CandidateId>,
    ) -> Result<(), ChoiceError> {
        if let Some(mut object) = self.get_object(identifier.clone()) {
            object.set_elected(elected_candidates);
            self.set_object(identifier, object)?;
            Ok(())
        } else {
            Err(ChoiceError::NotFound)
        }
    }
}

// pub trait ChoiceInterface {
//     type Object: ChoiceObject;

//     /// Retrieves a `ChoiceObject` by its identifier.
//     fn get_object(&self, identifier: ChoiceIdentifier) -> Option<Self::Object>;

//     /// Sets the `ChoiceObject` for a given identifier.
//     fn set_object(
//         &mut self,
//         identifier: ChoiceIdentifier,
//         object: Self::Object,
//     ) -> Result<(), ChoiceError>;

//     /// Updates the elected candidates for a `ChoiceObject` identified by `identifier`.
//     fn update_elected(
//         &mut self,
//         choice_id: ChoiceId,
//         identifier: ChoiceIdentifier,
//         elected_candidates: Vec<CandidateId>,
//     ) -> Result<(), ChoiceError> {
//         if let Some(mut object) = self.get_object(identifier.clone()) {
//             object.set_elected(elected_candidates);
//             self.set_object(identifier, object)?;
//             Ok(())
//         } else {
//             Err(ChoiceError::NotFound)
//         }
//     }
// }




// pub trait ChoiceInterface {
//     type Object: ChoiceObject;

//     /// Retrieves a `ChoiceObject` by its identifier.
//     fn get_object(&self, identifier: ChoiceIdentifier) -> &Self::Object;

//     /// Retrieves a mutable reference to a `ChoiceObject` by its identifier.
//     fn get_object_mut(&mut self, identifier: ChoiceIdentifier) -> &mut Self::Object;

//     /// Updates the elected candidates for a `ChoiceObject` identified by `identifier`.
//     fn update_elected(
//         &mut self,
//         choice_id: ChoiceId,
//         identifier: ChoiceIdentifier,
//         elected_candidates: Vec<CandidateId>,
//     ) -> Result<(), ChoiceError> {
//         let object = self.get_object_mut(identifier);
//         object.set_elected(elected_candidates);
//         Ok(())
//     }
// }

pub trait ChoiceObject {
    /// Sets the elected items.
    fn set_elected(&mut self, elected: Vec<CandidateId>);
}


// pub trait ChoiceInterface {
//     type Object: ChoiceObject;

//     /// Retrieves a ChoiceObject by its identifier.
//     fn get_object(&self, identifier: ChoiceId) -> &Self::Object;

//     /// Retrieves a mutable reference to a ChoiceObject by its identifier.
//     fn get_object_mut(&mut self, identifier: ChoiceId) -> &mut Self::Object;

//     /// Updates the votes for a ChoiceObject identified by `identifier`.
//     fn update_votes(
//         &mut self,
//         identifier: ChoiceId,
//         choice_vote: ChoiceVote,
//     ) -> Result<(), ChoiceError> {
//         let object = self.get_object_mut(identifier);
//         object.update_votes(choice_vote)
//     }
// }

// pub trait ChoiceObject {
//     type Item: ChoiceItem;

//     /// Retrieves a candidate by identifier.
//     fn get_candidate(&self, id: &CandidateId) -> Option<Self::Item>;

//     /// Sets a candidate in the choice object.
//     fn set_candidate(&mut self, id: &CandidateId, item: Self::Item);

//     /// Retrieves the currently elected items.
//     fn get_elected(&self) -> Vec<Self::Item>;

//     /// Sets the elected items.
//     fn set_elected(&mut self, elected: Vec<Self::Item>);

//     /// Retrieves all candidates in the choice object.
//     fn candidates(&self) -> Vec<Self::Item>;

//     /// Retrieves the maximum number of options a user can vote for.
//     fn max_vote_options(&self) -> u8;

//     /// Retrieves the number of items to be elected.
//     fn num_elected_items(&self) -> u8;

//     /// Retrieves the previous vote of a user, if any.
//     fn get_previous_vote(&self, account_id: &AccountId) -> Option<ChoiceVote>;

//     /// Sets the vote for a user.
//     fn set_vote(&mut self, account_id: &AccountId, vote: ChoiceVote);

//     /// Performs the vote update operation, handling multiple options and electing items.
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
//                     .ok_or_else(|| ChoiceError::CandidateNotFound(format!("{:?}", option_id)))?;

//                 let new_vote_count = candidate
//                     .get_vote_count()
//                     .checked_sub(previous_vote.weight)
//                     .ok_or_else(|| ChoiceError::Underflow(format!("{:?}", option_id)))?;
//                 candidate.set_vote_count(new_vote_count);

//                 self.set_candidate(&option_id, candidate);
//             }
//         }

//         // Add new vote weights
//         for option_id in &choice_vote.voted_options {
//             let mut candidate = self
//                 .get_candidate(option_id)
//                 .ok_or_else(|| ChoiceError::CandidateNotFound(format!("{:?}", option_id)))?;

//             let new_vote_count = candidate
//                 .get_vote_count()
//                 .checked_add(choice_vote.weight)
//                 .ok_or_else(|| ChoiceError::Overflow(format!("{:?}", option_id)))?;
//             candidate.set_vote_count(new_vote_count);

//             self.set_candidate(option_id, candidate);
//         }

//         // Store the new vote
//         self.set_vote(&account_id, choice_vote);

//         // Recalculate the elected items
//         let mut candidates = self.candidates();

//         // Sort candidates by vote count in descending order
//         candidates.sort_by(|a, b| b.get_vote_count().cmp(&a.get_vote_count()));

//         let num_elected = self.num_elected_items() as usize;

//         // For single or multiple elections, take the top N candidates
//         let elected_candidates = candidates.into_iter().take(num_elected).collect();

//         // Update the elected items
//         self.set_elected(elected_candidates);

//         Ok(())
//     }
// }



// pub trait ChoiceItem {
//     /// Gets the vote count or score for the item.
//     fn get_vote_count(&self) -> u128;

//     /// Sets the vote count or score for the item.
//     fn set_vote_count(&mut self, count: u128);

//     /// Gets the identifier of the item (e.g., option ID).
//     fn get_identifier(&self) -> CandidateId;
// }

#[derive(Debug)]
pub enum ChoiceError {
    InvalidVote(String),
    CandidateNotFound(String),
    Overflow(String),
    Underflow(String),
    Unauthorized,
    NotFound,
    InvalidGroupKind,
    InvalidIdentifier,
    // Add other relevant error variants
}