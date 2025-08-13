use crate::*;
use near_sdk::json_types::U128;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct RuleInput {
    pub dao_id: DaoId,
    pub flag_role_id: RoleId,
    pub jury_role_id: RoleId,
    // pub required_flag_count: u64,
    pub penalty_function: Option<FunctionCall>,
    pub penalty_amount: Amount,
    pub policy: RulePolicy,
    // Video fields
    pub name: String,
    pub description: Option<String>,
    pub video: VideoHash,
    pub image: ImageHash,
    pub location: Option<String>,
}

impl Default for RuleInput {
    fn default() -> Self {
        Self::example()
    }
}

impl RuleInput {
    /// Generate an example RuleInput
    pub fn example() -> Self {
        Self {
            dao_id: "example-dao.near".parse().unwrap(),
            flag_role_id: 0, // Followers role
            jury_role_id: 1, // Council role
            penalty_function: Some(FunctionCall {
                contract_id: "penalty.example-dao.near".parse().unwrap(),
                method_name: "apply_penalty".to_string(),
                args: near_sdk::json_types::Base64VecU8::from(vec![]),
                deposit: near_sdk::json_types::U128(0),
                gas: near_sdk::json_types::U64(30_000_000_000_000),
            }),
            penalty_amount: Amount {
                token_id: "token.example-dao.near".parse().unwrap(),
                amount: U128(100),
            },
            policy: RulePolicy {
                flag_threshold: 300, // 3% threshold (PercentageU32 is in basis points)
                flag_quorum: 10, // Need 10 flags minimum
                review_threshold: 6000, // 60% approval needed (in basis points)
                review_quorum: Quorum::Percentage(20), // 20% of jury must participate
            },
            name: "No Spam".to_string(),
            description: Some("Prohibits spam content in the DAO".to_string()),
            video: "QmExampleVideoHash".to_string(),
            image: "QmExampleImageHash".to_string(),
            location: None,
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct RuleInputVideoOption {
    pub dao_id: DaoId,
    pub flag_role_id: RoleId,
    pub jury_role_id: RoleId,
    // pub required_flag_count: u64,
    pub penalty_function: Option<FunctionCall>,
    pub penalty_amount: Amount,
    pub policy: RulePolicy,
    // Video fields
    pub name: String,
    pub description: Option<String>,
    pub video_bundle: Option<VideoBundle>,
    pub location: Option<String>,
}

impl RuleInput {

    pub fn from_video_option(
        input: RuleInputVideoOption,
        proposal_video: VideoHash,
        proposal_image: ImageHash,
    ) -> Self {
        let (video, image) = if let Some(bundle) = input.video_bundle {
            (bundle.video, bundle.image)
        } else {
            (proposal_video, proposal_image)
        };
        Self {
            dao_id: input.dao_id,
            flag_role_id: input.flag_role_id,
            jury_role_id: input.jury_role_id,
            // required_flag_count: input.required_flag_count,
            penalty_function: input.penalty_function,
            penalty_amount: input.penalty_amount,
            policy: input.policy,
            name: input.name,
            description: input.description,
            video,
            image,
            location: input.location,
        }
    }

    pub fn unwrap_video_option(input: RuleInputVideoOption) -> Self {

        let video_bundle = input.video_bundle.unwrap();

        Self {
            dao_id: input.dao_id,
            flag_role_id: input.flag_role_id,
            jury_role_id: input.jury_role_id,
            // required_flag_count: input.required_flag_count,
            penalty_function: input.penalty_function,
            penalty_amount: input.penalty_amount,
            policy: input.policy,
            name: input.name,
            description: input.description,
            video: video_bundle.video,
            image: video_bundle.image,
            location: None,
        }
    }
}
