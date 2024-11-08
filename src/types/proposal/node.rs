use crate::*;
use near_sdk::serde_json;
use neo4rs::Node;

impl Proposal {
    pub fn from_node(node: Node) -> Self {
        // Extract account IDs
        let dao_id = match node.get::<String>("dao_id") {
            Ok(id_str) => parse_account_id(id_str),
            Err(_) => "default.near".parse::<AccountId>().unwrap(),
        };
        let proposer_id = match node.get::<String>("proposer_id") {
            Ok(id_str) => parse_account_id(id_str),
            Err(_) => "default.near".parse::<AccountId>().unwrap(),
        };
        
        let proposer_group_id = node.get::<GroupId>("proposer_group_id").unwrap_or_default();

        // Extract other fields
        let id = node.get::<u64>("id").unwrap_or_default();
        let kind = match node.get::<String>("kind") {
            Ok(kind_str) => serde_json::from_str(&kind_str).unwrap_or(ProposalKind::Vote),
            Err(_) => ProposalKind::Vote,
        };
        let video = node.get::<String>("video").unwrap_or_default();
        let image = node.get::<String>("image").unwrap_or_default();
        let description = node.get::<String>("description").unwrap_or_default();
        
        // Deserialize voting_sessions
        let voting_sessions = match node.get::<String>("voting_sessions") {
            Ok(sessions_str) => serde_json::from_str(&sessions_str).unwrap_or_default(),
            Err(_) => vec![],
        };

        // Extract status as JSON then parse to ProposalStatus
        let status = match node.get::<String>("status") {
            Ok(status_str) => serde_json::from_str(&status_str).unwrap_or(ProposalStatus::Initializing),
            Err(_) => ProposalStatus::Initializing,
        };

        let submission_time = node.get::<i64>("submission_time").unwrap_or(0) as u64;
        let bond = node.get::<u128>("bond").unwrap_or_default();

        Proposal {
            id,
            dao_id,
            proposer_id,
            proposer_group_id,
            kind,
            video,
            image,
            description,
            voting_sessions,
            status,
            submission_time,
            bond,
        }
    }
}
