use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProfileOutput {
    pub profile: Profile,
    pub engagement: ProfileEngagement,
    pub interaction: ProfileInteraction,
    pub mutual_connections: MutualConnections,
}