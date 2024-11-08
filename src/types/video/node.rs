use crate::*;
use near_sdk::serde_json;
use neo4rs::Node;

impl Video {
    pub fn from_node(node: Node) -> Self {
        // Extract owner_id
        let owner_id = match node.get::<String>("owner_id") {
            Ok(id_str) => parse_account_id(id_str),
            Err(_) => "default.near".parse::<AccountId>().unwrap(),
        };

        // Extract creator_id
        let creator_id = match node.get::<String>("creator_id") {
            Ok(id_str) => parse_account_id(id_str),
            Err(_) => "default.near".parse::<AccountId>().unwrap(),
        };

        // Extract other fields
        let id = node.get::<VideoId>("id").unwrap_or_default();
        let title = node.get::<String>("title").unwrap_or_default();
        let caption = node.get::<String>("caption").ok();
        let video = node.get::<String>("video").unwrap_or_default();
        let image = node.get::<String>("image").unwrap_or_default();
        let location = node.get::<String>("location").ok();
        let issued_at = node.get::<i64>("issued_at").unwrap_or(0) as u64;
        let updated_at = node.get::<i64>("updated_at").map(|t| t as u64).ok();

        // Deserialize permissions
        let permissions = match node.get::<String>("permissions") {
            Ok(perm_str) => match serde_json::from_str(&perm_str) {
                Ok(perm) => perm,
                Err(_) => VideoPermissions::default(),
            },
            Err(_) => VideoPermissions::default(),
        };

        // Deserialize tags
        let tags = match node.get::<String>("tags") {
            Ok(tags_str) => match serde_json::from_str(&tags_str) {
                Ok(tags) => Some(tags),
                Err(_) => None,
            },
            Err(_) => None,
        };

        // Deserialize royalty
        let royalty = match node.get::<String>("royalty") {
            Ok(royalty_str) => match serde_json::from_str(&royalty_str) {
                Ok(royalty) => Some(royalty),
                Err(_) => None,
            },
            Err(_) => None,
        };

        Video {
            id,
            owner_id,
            creator_id,
            title,
            caption,
            video,
            image,
            location,
            issued_at,
            updated_at,
            permissions,
            tags,
            royalty,
        }
    }
}