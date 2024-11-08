use crate::*;
use near_sdk::serde_json;
use neo4rs::Node;

impl Group {
    pub fn from_node(node: Node) -> Self {
        // Extract account IDs
        let dao_id = match node.get::<String>("dao_id") {
            Ok(id_str) => parse_account_id(id_str),
            Err(_) => "default.near".parse::<AccountId>().unwrap(),
        };

        // Extract other fields
        let id = node.get::<GroupId>("id").unwrap_or_default();
        let name = node.get::<String>("name").unwrap_or_default();

        // Deserialize GroupKind
        let kind = match node.get::<String>("kind") {
            Ok(kind_str) => serde_json::from_str(&kind_str).unwrap_or(GroupKind::Followers),
            Err(_) => GroupKind::Followers,
        };

        // Deserialize VoteWeightKind
        let vote_weight = match node.get::<String>("vote_weight") {
            Ok(weight_str) => serde_json::from_str(&weight_str).unwrap_or(VoteWeightKind::Single),
            Err(_) => VoteWeightKind::Single,
        };

        // Deserialize permissions
        let permissions = match node.get::<String>("permissions") {
            Ok(perms_str) => serde_json::from_str(&perms_str).unwrap_or_default(),
            Err(_) => HashMap::new(),
        };

        Group {
            id,
            dao_id,
            name,
            kind,
            vote_weight,
            permissions,
        }
    }
}
