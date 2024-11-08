use crate::*;
use near_sdk::serde_json;
use neo4rs::Node;

impl Indictment {
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

        // Deserialize accusation_ids
        let accusation_ids = match node.get::<String>("accusation_ids") {
            Ok(acc_ids_str) => serde_json::from_str(&acc_ids_str).unwrap_or_default(),
            Err(_) => Vec::new(),
        };

        // Deserialize evidence
        let evidence = match node.get::<String>("evidence") {
            Ok(evidence_str) => serde_json::from_str(&evidence_str).unwrap_or_default(),
            Err(_) => Vec::new(),
        };

        // Deserialize violation_time
        let violation_time = match node.get::<String>("violation_time") {
            Ok(violation_str) => serde_json::from_str(&violation_str).unwrap_or(ViolationTime::Unknown),
            Err(_) => ViolationTime::Unknown,
        };

        // Deserialize status
        let status = match node.get::<String>("status") {
            Ok(status_str) => serde_json::from_str(&status_str).unwrap_or(IndictmentStatus::Open),
            Err(_) => IndictmentStatus::Open,
        };

        Indictment {
            id: node.get::<IndictmentId>("id").unwrap_or_default(),
            accusation_ids,
            law_id: node.get::<LawId>("law_id").unwrap_or_default(),
            dao_id,
            accused_id,
            evidence,
            status,
            violation_time,
            initiated_at: node.get::<TimestampSeconds>("initiated_at").unwrap_or(0),
        }
    }
}
