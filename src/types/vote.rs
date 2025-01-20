// use crate::*;
// // use neo4rs::Node;

// pub enum VoteKind {
//     Proposal(Proposal),
//     Choice(Choice),
//     Calibration(Calibration),
// }

// // impl VoteKind {
// //     pub fn from_node(node: Node) -> Self {
// //         // Check if the node has a "type" property indicating which variant it is
// //         let item_type = node.get::<String>("type").unwrap_or_default();

// //         match item_type.as_str() {
// //             "Proposal" => {
// //                 // If it's a Proposal, parse the Proposal fields and create a Proposal instance
// //                 let proposal = Proposal::from_node(node);
// //                 VoteKind::Proposal(proposal)
// //             }
// //             "Choice" => {
// //                 // If it's a Choice, parse the Choice fields and create a Choice instance
// //                 let choice = Choice::from_node(node);
// //                 VoteKind::Choice(choice)
// //             }
// //             "Calibration" => {
// //                 // If it's a Calibration, parse the Calibration fields and create a Calibration instance
// //                 let calibration = Calibration::from_node(node);
// //                 VoteKind::Calibration(calibration)
// //             }
// //             _ => {
// //                 panic!("Unexpected type in VoteItem: '{}'", item_type);
// //             }
// //         }
// //     }
// // }

// pub enum VoteKindOutput {
//     Proposal(ProposalOutput),
//     Choice(ChoiceOutput),
//     Calibration(CalibrationOutput),
// }