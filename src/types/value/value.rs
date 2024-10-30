use crate::*;

pub type ValueId = u64;
pub type ValueName = String;
pub type SubValueName = String;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Value {
    pub id: ValueId,
    pub name: ValueName,
    pub description: String,
    pub dao_id: DaoId,
    pub structure: ValueStructure,
    pub vote_method: VoteMethod, // Rename to change_method - Oct 2 2024
    pub calling_contract: ContractId,
}

impl Value {
    pub fn from_input(input: ValueInput, id: ValueId, choice_id: Option<ChoiceId>) -> Self {
        // Match the vote_method_input and ensure ChoiceId is provided if necessary
        let vote_method = match input.vote_method_input {
            VoteMethodInput::Proposal => VoteMethod::Proposal,
            VoteMethodInput::Choice(_, _, _) => {
                if let Some(choice_id) = choice_id {
                    VoteMethod::Choice(choice_id)
                } else {
                    panic!("ERR_CHOICE_ID: ChoiceId is required for VoteMethod::Choice");
                }
            },
        };

        // Construct the Value object based on the input fields
        Value {
            id,                          
            name: input.name,     
            description: input.description,       
            dao_id: env::predecessor_account_id(),
            structure: input.structure,
            vote_method,      
            calling_contract: input.calling_contract,
        }
    }
}
