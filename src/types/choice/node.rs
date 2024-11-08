use crate::*;
use near_sdk::serde_json;
use neo4rs::Node;

impl Choice {
    pub fn from_node(node: Node) -> Self {
        // Extract account IDs
        let dao_id = match node.get::<String>("dao_id") {
            Ok(id_str) => parse_account_id(id_str),
            Err(_) => "default.near".parse::<AccountId>().unwrap(),
        };

        let group_id = node.get::<GroupId>("group_id").unwrap_or_default();

        // Extract other fields
        let id = node.get::<ChoiceId>("id").unwrap_or_default();
        let description = node.get::<String>("description").unwrap_or_default();
        let video = node.get::<String>("video").unwrap_or_default();
        let image = node.get::<String>("image").unwrap_or_default();
        let size = node.get::<u8>("size").unwrap_or_default();
        let max_vote_options = node.get::<u8>("max_vote_options").unwrap_or_default();
        let initiated_at = node.get::<i64>("initiated_at").unwrap_or(0) as u64;

        // Deserialize kind
        let kind = match node.get::<String>("kind") {
            Ok(kind_str) => serde_json::from_str(&kind_str).unwrap_or(ChoiceKind::Value(0)),
            Err(_) => ChoiceKind::Value(0),
        };

        Choice {
            id,
            dao_id,
            group_id,
            kind,
            video,
            image,
            description,
            size,
            max_vote_options,
            initiated_at,
        }
    }
}
