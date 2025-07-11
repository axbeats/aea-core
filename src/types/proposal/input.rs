use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProposalInput {
    // Proposal fields  
    pub dao_id: AccountId,
    pub role_id: RoleId,
    pub action: ProposalAction,
    pub bond: u128,
    // Video fields
    pub title: String,        
    pub description: Option<String>,           
    pub video: VideoHash,                
    pub image: ImageHash,             
    pub location: Option<String>,
    pub user_location: Option<Point>,  // User's coordinates for region verification
}