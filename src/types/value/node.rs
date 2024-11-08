use crate::*;
use near_sdk::serde_json;
use neo4rs::Node;

impl Value {
    pub fn from_node(node: Node) -> Self {
        // Extract account IDs
        let dao_id = match node.get::<String>("dao_id") {
            Ok(id_str) => parse_account_id(id_str),
            Err(_) => "default.near".parse::<AccountId>().unwrap(),
        };

        // Extract other fields
        let id = node.get::<ValueId>("id").unwrap_or_default();
        let name = node.get::<String>("name").unwrap_or_default();
        let description = node.get::<String>("description").unwrap_or_default();

        // Deserialize ValueStructure
        let structure = match node.get::<String>("structure") {
            Ok(struct_str) => serde_json::from_str(&struct_str).unwrap_or(ValueStructure::Single(SingleValue {
                value: ValueType::String("".to_string()),
                validation: None,
            })),
            Err(_) => ValueStructure::Single(SingleValue {
                value: ValueType::String("".to_string()),
                validation: None,
            }),
        };

        // Deserialize VoteMethod
        let vote_method = match node.get::<String>("vote_method") {
            Ok(vote_method_str) => serde_json::from_str(&vote_method_str).unwrap_or(VoteMethod::Proposal),
            Err(_) => VoteMethod::Proposal,
        };

        // Deserialize calling_contract
        let calling_contract = match node.get::<String>("calling_contract") {
            Ok(contract_str) => parse_account_id(contract_str),
            Err(_) => "default.near".parse::<AccountId>().unwrap(),
        };

        Value {
            id,
            dao_id,
            name,
            description,
            structure,
            vote_method,
            calling_contract,
        }
    }
}
