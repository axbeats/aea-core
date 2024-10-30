use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum VoteMethod {
    Proposal,
    Choice(ChoiceId),
}

// Vote method as a parameter input for Proposal indicating the Group that can vote on the choice.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum VoteMethodInput {
    Proposal,
    Choice(GroupId, VideoHash, ImageHash),
}

impl VoteMethod {
    pub fn to_policy_label(&self) -> &'static str {
        match self {
            VoteMethod::Proposal => "proposal",
            VoteMethod::Choice(_) => "choice",
        }
    }
}

impl Default for VoteMethod {
    fn default() -> Self {
        VoteMethod::Proposal
    }
}