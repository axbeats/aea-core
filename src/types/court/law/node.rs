use crate::*;
use near_sdk::serde_json;
use neo4rs::Node;

impl Law {
    pub fn from_node(node: Node) -> Self {
        // Extract account IDs
        let dao_id = match node.get::<String>("dao_id") {
            Ok(id_str) => parse_account_id(id_str),
            Err(_) => "default.near".parse::<AccountId>().unwrap(),
        };

        // Extract other fields
        let id = node.get::<LawId>("id").unwrap_or_default();
        let name = node.get::<String>("name").unwrap_or_default();
        let video = node.get::<String>("video").unwrap_or_default();
        let image = node.get::<String>("image").unwrap_or_default();
        let description = node.get::<String>("description").unwrap_or_default();
        let accusation_group_id = node
            .get::<GroupId>("accusation_group_id")
            .unwrap_or_default();
        let jury_group_id = node.get::<GroupId>("jury_group_id").unwrap_or_default();
        let required_accusation_count = node.get::<u64>("required_accusation_count").unwrap_or(0);
        let initiated_at = node.get::<i64>("initiated_at").unwrap_or(0) as u64;

        // Deserialize penalty_function as an Option<FunctionCall>
        let penalty_function = match node.get::<String>("penalty_function") {
            Ok(func_str) => serde_json::from_str(&func_str).ok(),
            Err(_) => None,
        };

        // Deserialize penalty_amount as Amount
        let penalty_amount = match node.get::<String>("penalty_amount") {
            Ok(amount_str) => serde_json::from_str(&amount_str).unwrap_or(Amount {
                token_id: "default_token.near".parse().unwrap(),
                amount: 0,
            }),
            Err(_) => Amount {
                token_id: "default_token.near".parse().unwrap(),
                amount: 0,
            },
        };

        // Deserialize policy as LawPolicy
        let policy = match node.get::<String>("policy") {
            Ok(policy_str) => serde_json::from_str(&policy_str).unwrap_or(LawPolicy {
                threshold: 0,
                quorum: Quorum::Fixed(0),
            }),
            Err(_) => LawPolicy {
                threshold: 0,
                quorum: Quorum::Fixed(0),
            },
        };

        Law {
            id,
            dao_id,
            name,
            video,
            image,
            description,
            accusation_group_id,
            jury_group_id,
            required_accusation_count,
            penalty_function,
            penalty_amount,
            policy,
            initiated_at,
        }
    }
}
