use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum VoteMethod {
    Proposal,
    Choice(ChoiceId),
    Calibration(CalibrationId),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum VoteMethodInput {
    Proposal,
    Choice(ChoiceInput),
    Calibration(CalibrationInput),
}

impl VoteMethodInput {
    pub fn is_proposal(&self) -> bool {
        matches!(self, VoteMethodInput::Proposal)
    }

    pub fn is_choice(&self) -> bool {
        matches!(self, VoteMethodInput::Choice(_))
    }

    pub fn is_calibration(&self) -> bool {
        matches!(self, VoteMethodInput::Calibration(_))
    }

    pub fn as_choice(&self) -> Option<&ChoiceInput> {
        match self {
            VoteMethodInput::Choice(input) => Some(input),
            _ => None,
        }
    }

    pub fn as_calibration(&self) -> Option<&CalibrationInput> {
        match self {
            VoteMethodInput::Calibration(input) => Some(input),
            _ => None,
        }
    }

    // Panicking versions when you know the type at compile time
    pub fn expect_choice(&self) -> &ChoiceInput {
        match self {
            VoteMethodInput::Choice(input) => input,
            _ => panic!("Expected Choice input"),
        }
    }

    pub fn expect_calibration(&self) -> &CalibrationInput {
        match self {
            VoteMethodInput::Calibration(input) => input,
            _ => panic!("Expected Calibration input"),
        }
    }
}

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
// #[serde(crate = "near_sdk::serde")]
// pub enum VoteMethodInput {
//     Proposal,
//     Choice(GroupId, VideoHash, ImageHash),
// }

impl VoteMethod {
    pub fn to_policy_label(&self) -> &'static str {
        match self {
            VoteMethod::Proposal => "proposal",
            VoteMethod::Choice(_) => "choice",
            VoteMethod::Calibration(_) => "calibration",
        }
    }
}

impl Default for VoteMethod {
    fn default() -> Self {
        VoteMethod::Proposal
    }
}