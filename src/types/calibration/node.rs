use crate::*;
use near_sdk::serde_json;
use neo4rs::Node;

impl Calibration {
    pub fn from_node(node: Node) -> Self {
        // Extract account IDs
        let dao_id = match node.get::<String>("dao_id") {
            Ok(id_str) => parse_account_id(id_str),
            Err(_) => "default.near".parse::<AccountId>().unwrap(),
        };

        let group_id = node.get::<GroupId>("group_id").unwrap_or_default();
        let value_id = node.get::<ValueId>("value_id").unwrap_or_default();

        // Extract other fields
        let id = node.get::<CalibrationId>("id").unwrap_or_default();
        let description = node.get::<String>("description").unwrap_or_default();
        let video = node.get::<String>("video").unwrap_or_default();
        let image = node.get::<String>("image").unwrap_or_default();
        let cooldown_period = node.get::<i64>("cooldown_period").unwrap_or(0) as u64;
        let initiated_at = node.get::<i64>("initiated_at").unwrap_or(0) as u64;

        // Deserialize adjustment_factor
        let adjustment_factor = match node.get::<String>("adjustment_factor") {
            Ok(adjustment_str) => serde_json::from_str(&adjustment_str).unwrap_or(AdjustmentFactor::Fixed(0)),
            Err(_) => AdjustmentFactor::Fixed(0),
        };

        Calibration {
            id,
            value_id,
            dao_id,
            group_id,
            video,
            image,
            description,
            cooldown_period,
            adjustment_factor,
            initiated_at,
        }
    }
}
