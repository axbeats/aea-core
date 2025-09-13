use crate::*;

/// Actions that can be performed on token configuration
#[near(serializers = [json, borsh])]
#[serde(tag = "action", content = "params")]
#[derive(Debug, Clone, Generable)]
pub enum TokenAction {
    #[guide = "Change the name of a token-based role's token"]
    UpdateTokenName { 
        role_index: usize, 
        #[guide = "Full name of the token (e.g., 'DAO Governance Token')"]
        new_name: String 
    },
    #[guide = "Change the symbol of a token"]
    UpdateTokenSymbol { 
        role_index: usize, 
        #[guide = "Short symbol, typically 3-5 uppercase letters"]
        #[constraints = "max_length:10"]
        new_symbol: String 
    },
    #[guide = "Set the total supply of tokens"]
    UpdateTotalSupply { 
        role_index: usize, 
        #[guide = "Total number of tokens to mint (as string to avoid serialization issues)"]
        #[example = "1000000"]
        new_supply: String
    },
    #[guide = "Set decimal places for token display"]
    UpdateDecimals { 
        role_index: usize, 
        #[guide = "Number of decimal places (typically 18 for NEAR compatibility)"]
        #[constraints = "min:0,max:24"]
        new_decimals: u8 
    },
    #[guide = "Set the price for token sales"]
    UpdateTokenSalePrice { 
        role_index: usize, 
        #[guide = "Price per token in NEAR (as string)"]
        #[example = "0.1"]
        new_price: String
    },
    #[guide = "Set the payment currency for token sales"]
    UpdateTokenSalePaymentCurrency {
        role_index: usize,
        #[guide = "Currency to accept for payment (e.g., 'near', 'aea.near')"]
        #[example = "near"]
        new_currency: String
    },
    #[guide = "Set the deadline for token sales"]
    UpdateTokenSaleDeadline { 
        role_index: usize, 
        #[guide = "Days until deadline"]
        deadline_days: u32,
        #[guide = "Additional hours"]
        deadline_hours: u32,
        #[guide = "Additional minutes"]
        deadline_minutes: u32 
    },
}

impl Default for TokenAction {
    fn default() -> Self {
        Self::UpdateTokenName { role_index: 0, new_name: "".to_string() }
    }
}