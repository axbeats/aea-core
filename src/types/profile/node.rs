use crate::*;
use near_sdk::serde_json;
use neo4rs::Node;

impl Profile {
    pub fn from_node(node: Node) -> Self {
        // Extract id as AccountId
        let id = match node.get::<String>("id") {
            Ok(id_str) => parse_account_id(id_str),
            Err(_) => "default.near".parse::<AccountId>().unwrap(),
        };

        // Extract other fields
        let kind = match node.get::<String>("kind").as_deref() {
            Ok("Personal") => ProfileKind::Personal,
            Ok("Business") => ProfileKind::Business,
            Ok("Dao") => ProfileKind::Dao,
            _ => ProfileKind::Personal, // Default value if kind is not present or invalid
        };
        let name = node.get::<String>("name").unwrap_or_default();
        let bio = node.get::<String>("bio").ok();
        let image = node.get::<String>("image").unwrap_or_default();
        let video = node.get::<String>("video").ok();
        let link = node.get::<String>("link").ok();
        
        // Deserialize public_keys
        let public_keys = match node.get::<String>("public_keys") {
            Ok(keys_str) => match serde_json::from_str(&keys_str) {
                Ok(keys) => Some(keys),
                Err(_) => None,
            },
            Err(_) => None,
        };

        // Extract joined_date
        let joined_date = node.get::<i64>("joined_date").unwrap_or(0) as u64;

        Profile {
            id,
            kind,
            name,
            bio,
            image,
            video,
            link,
            public_keys,
            joined_date,
        }
    }
}
