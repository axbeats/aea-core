use crate::*;

/// Actions that can be performed on DAO roles
#[near(serializers = [json, borsh])]
#[serde(tag = "action", content = "params")]
#[derive(Debug, Clone, Generable)]
pub enum RoleAction {
    #[guide = "Create a new role for governance participation"]
    AddRole {
        #[guide = "Complete role configuration"]
        role: crate::RoleInput,
    },
    #[guide = "Remove an existing role"]
    RemoveRole {
        #[guide = "Zero-based index of the role to remove"]
        role_index: usize,
    },
    #[guide = "Rename an existing role"]
    UpdateRoleName { 
        #[guide = "Zero-based index of the role to update"]
        role_index: usize, 
        #[guide = "New display name for the role"]
        new_name: String 
    },
    #[guide = "Update the description of a role"]
    UpdateRoleDescription { 
        role_index: usize, 
        #[guide = "Detailed explanation of the role's purpose"]
        new_description: String 
    },
    #[guide = "Change the type of an existing role"]
    UpdateRoleKind { 
        role_index: usize, 
        #[constraints = "enum:Followers,Subscribers,Token,Elected,Region,Agent"]
        new_kind: String
    },
    #[guide = "Grant permission to create proposals of a specific type to a role"]
    AddCreatePermissions { 
        role_index: usize, 
        #[guide = "Proposal ability: Role, Policy, Task, Profile, Video, Code, Value, or Court"]
        #[constraints = "enum:Role,Policy,Task,Profile,Video,Code,Value,Court"]
        proposal_ability: String 
    },
    #[guide = "Remove permission to create proposals of a specific type from a role"]
    RemoveCreatePermissions { 
        role_index: usize, 
        #[guide = "Proposal ability: Role, Policy, Task, Profile, Video, Code, Value, or Court"]
        #[constraints = "enum:Role,Policy,Task,Profile,Video,Code,Value,Court"]
        proposal_ability: String
    },
    #[guide = "Grant voting permission on a proposal type to a role"]
    AddVotePermissions { 
        role_index: usize, 
        #[guide = "Proposal ability: Role, Policy, Task, Profile, Video, Code, Value, or Court"]
        #[constraints = "enum:Role,Policy,Task,Profile,Video,Code,Value,Court"]
        proposal_ability: String
    },
    #[guide = "Remove voting permission on a proposal type from a role"]
    RemoveVotePermissions { 
        role_index: usize, 
        #[constraints = "enum:Role,Policy,Task,Profile,Video,Code,Value,Court"]
        proposal_ability: String
    },
    #[guide = "Set the NEAR account for an Agent role"]
    UpdateAgentAccount { 
        role_index: usize, 
        #[guide = "NEAR account ID for the autonomous agent"]
        agent_account_id: String 
    },
    #[guide = "Update the region data for a Region role"]
    UpdateRegion {
        role_index: usize,
        #[guide = "Region data including name and boundaries"]
        region_input: crate::RegionRoleInput
    },
    #[guide = "Add members to an elected role"]
    AddElectedMembers {
        role_index: usize,
        #[guide = "Array of account IDs to add as members"]
        members: Vec<String>
    },
    #[guide = "Remove members from an elected role"]
    RemoveElectedMembers {
        role_index: usize,
        #[guide = "Array of account IDs to remove from members"]
        members: Vec<String>
    },
}

impl Default for RoleAction {
    fn default() -> Self {
        Self::AddRole {
            role: crate::RoleInput::default(),
        }
    }
}