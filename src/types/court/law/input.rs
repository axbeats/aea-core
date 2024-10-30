use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LawInput {
    pub dao_id: DaoId,
    pub name: String,
    pub video: VideoHash,
    pub image: ImageHash,
    pub description: String,
    pub accusation_group_id: GroupId,
    pub jury_group_id: GroupId,
    pub required_accusation_count: u64,
    pub penalty_function: Option<FunctionCall>,
    pub penalty_amount: Amount,
    pub policy: LawPolicy,
}