use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RuleInput {
    pub dao_id: DaoId,
    pub name: String,
    pub accusation_group_id: GroupId,
    pub jury_group_id: GroupId,
    pub required_accusation_count: u64,
    pub penalty_function: Option<FunctionCall>,
    pub penalty_amount: Amount,
    pub policy: RulePolicy,
    // Video fields
    pub title: String,
    pub description: Option<String>,
    pub video: VideoHash,
    pub image: ImageHash,
    pub location: Option<String>,
}
