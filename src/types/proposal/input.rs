use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProposalInput {
    // Proposal fields  
    pub dao_id: AccountId,
    pub group_id: GroupId,
    pub action: ProposalAction,
    pub bond: u128,
    // Video fields
    pub title: String,        
    pub description: Option<String>,           
    pub video: VideoHash,                
    pub image: ImageHash,             
    pub location: Option<String>,
}