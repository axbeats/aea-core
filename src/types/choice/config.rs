use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ChoiceConfig {
    pub value_id: ValueId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub kind: ChoiceKind,
    pub max_vote_options: u8,
    pub initial_values: Vec<String>,
}

impl From<ValueConfig> for ChoiceConfig {
    fn from(config: ValueConfig) -> Self {
        let (group_id, kind, max_vote_options) = match config.method_input {
            VoteMethodInput::Choice(choice_input) => {
                (choice_input.group_id, choice_input.kind, choice_input.max_vote_options)
            }
            _ => panic!("ValueConfig must have VoteMethodInput::Choice to convert into ChoiceConfig"),
        };

        Self {
            value_id: config.id,
            dao_id: config.dao_id,
            group_id,
            kind,
            max_vote_options,
            // If initial_values are not present in ValueConfig or its method_input, we default to empty
            initial_values: Vec::new(),
        }
    }
}

impl ChoiceConfig {
    pub fn from_input(
        input: ChoiceInput, 
        dao_id: DaoId, 
        value_id: ValueId, 
        initial_values: Vec<String>
    ) -> Self {
        Self {
            value_id,
            dao_id,
            group_id: input.group_id,
            kind: input.kind,
            max_vote_options: input.max_vote_options,
            initial_values,
        }
    }
}