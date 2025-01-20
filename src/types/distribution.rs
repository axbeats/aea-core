// use crate::*;

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
// #[serde(crate = "near_sdk::serde")]
// pub struct Distribution {
//     pub percentages: Vec<DistributionPercentage>,
// }

// impl Distribution {
//     pub fn new(percentages: Vec<DistributionPercentage>) -> Result<Self, Error> {
//         let distribution = Self { percentages };
//         if distribution.validate() {
//             Ok(distribution)
//         } else {
//             Err(anyhow!("The total percentage must equal 100, but got {}", distribution.total_percentage()))
//         }
//     }

//     pub fn validate(&self) -> bool {
//         let total: YoctoNumber = self.percentages.iter().map(|p| p.percentage).sum();
//         total == ONE_HUNDRED_YOCTO_NUMBER
//     }

//     fn total_percentage(&self) -> YoctoNumber {
//         self.percentages.iter().map(|p| p.percentage).sum()
//     }

//     pub fn calibrate_distribution(
//         &mut self,
//         increase: String,
//         decrease: String,
//         adjustment_amount: YoctoNumber,
//     ) -> Result<(), Error> {
//         // Find the indices of the increase and decrease entries
//         let increase_index = self
//             .percentages
//             .iter()
//             .position(|p| p.name == increase)
//             .ok_or_else(|| anyhow!("The increase entry with name '{}' does not exist", increase))?;
        
//         let decrease_index = self
//             .percentages
//             .iter()
//             .position(|p| p.name == decrease)
//             .ok_or_else(|| anyhow!("The decrease entry with name '{}' does not exist", decrease))?;
        
//         // Ensure we are not modifying the same entry
//         if increase_index == decrease_index {
//             return Err(anyhow!("Increase and decrease entries must be different"));
//         }

//         // Ensure the decrease entry has enough to adjust
//         if self.percentages[decrease_index].percentage < adjustment_amount {
//             return Err(anyhow!(
//                 "Cannot decrease '{}' by {}, insufficient percentage",
//                 decrease,
//                 adjustment_amount
//             ));
//         }

//         // Apply the adjustment
//         self.percentages[increase_index].percentage = self.percentages[increase_index]
//             .percentage
//             .checked_add(adjustment_amount)
//             .ok_or_else(|| anyhow!("Adding adjustment to '{}' would cause overflow", increase))?;

//         self.percentages[decrease_index].percentage = self.percentages[decrease_index]
//             .percentage
//             .checked_sub(adjustment_amount)
//             .ok_or_else(|| {
//                 anyhow!("Subtracting adjustment from '{}' would cause underflow", decrease)
//             })?;

//         // Validate the total after adjustment
//         if !self.validate() {
//             return Err(anyhow!(
//                 "Adjustment results in an invalid distribution total; it must sum to 100"
//             ));
//         }

//         Ok(())
//     }

//     pub fn add_percentage(
//         &mut self,
//         name: String,
//         percentage: YoctoNumber,
//         sub_distribution_id: ValueId,
//     ) -> Result<(), Error> {
//         // Validate that the new percentage doesn't exceed 100 when added
//         let total_percentage = self.total_percentage();
//         if total_percentage + percentage > ONE_HUNDRED_YOCTO_NUMBER {
//             return Err(anyhow!(
//                 "Adding this percentage would exceed the total of 100, currently at {}",
//                 total_percentage
//             ));
//         }

//         // Ensure no duplicate name
//         if self.percentages.iter().any(|p| p.name == name) {
//             return Err(anyhow!("Percentage with name '{}' already exists", name));
//         }

//         // Calculate the adjustment for each existing percentage
//         let num_existing = self.percentages.len();
//         let decrement_share = percentage / num_existing as u128;
//         let leftover = percentage % num_existing as u128;

//         // Subtract the decremented share from each existing percentage
//         for (i, p) in self.percentages.iter_mut().enumerate() {
//             p.percentage = p.percentage.checked_sub(decrement_share).ok_or_else(|| {
//                 anyhow!("Error: Decrementing percentage would result in a negative value")
//             })?;
//             if i == 0 {
//                 // Adjust for any leftover percentage
//                 p.percentage = p.percentage.checked_sub(leftover).ok_or_else(|| {
//                     anyhow!("Error: Leftover decrement results in negative percentage")
//                 })?;
//             }
//         }

//         // Add the new DistributionPercentage entry with the specified percentage
//         self.percentages.push(DistributionPercentage {
//             name,
//             percentage,
//             sub_distribution_id: Some(sub_distribution_id),
//         });

//         Ok(())
//     }

//     pub fn add_percentage_zero_value(&mut self, name: String, sub_distribution_id: ValueId) -> Result<(), Error> {
//         if self.percentages.iter().any(|p| p.name == name) {
//             return Err(anyhow!("Percentage with name '{}' already exists", name));
//         }

//         // Add the new DistributionPercentage with 0% allocation
//         self.percentages.push(DistributionPercentage {
//             name,
//             percentage: 0,
//             sub_distribution_id: Some(sub_distribution_id),
//         });

//         Ok(())
//     }

//     pub fn remove_percentage(&mut self, name: String) -> Result<(), Error> {
//         let index = self.percentages.iter().position(|p| p.name == name);
        
//         // Ensure the name exists in the distribution
//         let index = match index {
//             Some(idx) => idx,
//             None => return Err(anyhow!("Percentage with name '{}' not found", name)),
//         };

//         // Get the percentage to distribute and remove the item
//         let removed_percentage = self.percentages[index].percentage;
//         self.percentages.remove(index);

//         // Distribute the removed percentage equally among the remaining percentages
//         let num_remaining = self.percentages.len();
//         if num_remaining == 0 {
//             return Err(anyhow!("Cannot remove the only remaining percentage"));
//         }

//         let distribution_share = removed_percentage / num_remaining as u128;
//         let leftover = removed_percentage % num_remaining as u128;

//         for (i, p) in self.percentages.iter_mut().enumerate() {
//             p.percentage += distribution_share;
//             if i == 0 {
//                 // Distribute any leftover to the first entry
//                 p.percentage += leftover;
//             }
//         }

//         Ok(())
//     }

// }

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
// #[serde(crate = "near_sdk::serde")]
// pub struct DistributionPercentage {
//     pub name: String,
//     pub percentage: YoctoNumber,
//     pub sub_distribution_id: Option<ValueId>,
// }
