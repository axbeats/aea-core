use crate::*;

/// Actions that can be performed on DAO policy
#[near(serializers = [json, borsh])]
#[serde(tag = "action", content = "params")]
#[derive(Debug, Clone, Generable)]
pub enum PolicyAction {
    #[guide = "Set the percentage of votes needed to pass a proposal"]
    UpdateThreshold { 
        #[guide = "Percentage from 50 to 100"]
        #[constraints = "min:50,max:100"]
        new_threshold: u8 
    },
    #[guide = "Set early approval threshold for quick decisions"]
    UpdateEarlyThreshold { 
        #[guide = "Optional higher threshold for early approval"]
        new_early_threshold: Option<u8> 
    },
    #[guide = "Set minimum participation percentage"]
    UpdateQuorum { 
        #[guide = "Percentage from 1 to 100"]
        #[constraints = "min:1,max:100"]
        new_quorum: u8 
    },
    #[guide = "Set early quorum for quick decisions"]
    UpdateEarlyQuorum { 
        new_early_quorum: Option<u8> 
    },
    #[guide = "Set how long voting remains open"]
    UpdateVotingPeriod { 
        #[guide = "Number of days"]
        days: u32, 
        #[guide = "Additional hours"]
        hours: u32, 
        #[guide = "Additional minutes"]
        minutes: u32 
    },
    #[guide = "Set the cost to create a proposal"]
    UpdateBond { 
        #[guide = "Amount required to create a proposal (as string)"]
        #[example = "1"]
        new_bond: String,
        #[guide = "Currency for the bond"]
        currency: BondCurrency 
    },
}

impl Default for PolicyAction {
    fn default() -> Self {
        Self::UpdateThreshold { new_threshold: 50 }
    }
}