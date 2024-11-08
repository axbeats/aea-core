use crate::*;
use neo4rs::Node;
use near_sdk::serde_json;

impl Accusation {
    pub fn from_node(node: Node) -> Self {
        // Extract dao_id with a fallback
        let dao_id = match node.get::<String>("dao_id") {
            Ok(id_str) => parse_account_id(id_str),
            Err(_) => "default.near".parse::<AccountId>().unwrap(),
        };

        // Extract accused_id
        let accused_id = match node.get::<String>("accused_id") {
            Ok(id_str) => parse_account_id(id_str),
            Err(_) => "default.near".parse::<AccountId>().unwrap(),
        };

        // Extract accuser_id
        let accuser_id = match node.get::<String>("accuser_id") {
            Ok(id_str) => parse_account_id(id_str),
            Err(_) => "default.near".parse::<AccountId>().unwrap(),
        };

        // Deserialize Evidence
        let evidence = match node.get::<String>("evidence") {
            Ok(evidence_str) => serde_json::from_str(&evidence_str).unwrap_or(Evidence::Video(VideoHash::default())),
            Err(_) => Evidence::Video(VideoHash::default()),
        };

        // Deserialize ViolationTime
        let violation_time = match node.get::<String>("violation_time") {
            Ok(violation_str) => serde_json::from_str(&violation_str).unwrap_or(ViolationTime::Unknown),
            Err(_) => ViolationTime::Unknown,
        };

        Accusation {
            id: node.get::<AccusationId>("id").unwrap_or_default(),
            law_id: node.get::<LawId>("law_id").unwrap_or_default(),
            dao_id,
            accused_id,
            accuser_id,
            evidence,
            violation_time,
            reported_at: node.get::<TimestampMilliSeconds>("reported_at").unwrap_or(0),
        }
    }
}
