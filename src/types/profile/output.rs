use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProfileOutput {
    pub profile: Profile,
    pub engagement: ProfileEngagement,
    pub interaction: ProfileInteraction,
    pub mutual_connections: MutualConnections,
}