use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct GovernedValueId {
    pub contract_id: ContractId,
    pub field_id: FieldId,
    pub identifier: Option<Identifier>,
}

impl GovernedValueId {
    pub fn new(contract_id: ContractId, field_id: FieldId, identifier: Option<Identifier>) -> Self {
        Self { 
            contract_id, 
            field_id, 
            identifier }
    }
}

impl GovernedValueId {
    pub fn identifier(&self) -> Option<&Identifier> {
        self.identifier.as_ref()
    }

    pub fn expect_identifier(&self) -> &Identifier {
        self.identifier.as_ref()
            .expect("GovernedValueId: identifier is required but was None")
    }

    pub fn expect_no_identifier(&self) {
        if self.identifier.is_some() {
            panic!("GovernedValueId: identifier should be None but was Some");
        }
    }

    // Convenience methods to directly get the typed value
    pub fn get_string(&self) -> Option<&String> {
        self.identifier()?.as_string()
    }

    pub fn get_account_id(&self) -> Option<&AccountId> {
        self.identifier()?.as_account_id()
    }

    pub fn get_u64(&self) -> Option<u64> {
        self.identifier()?.as_u64()
    }

    // Panicking versions when you know the type and that identifier exists
    pub fn expect_string(&self) -> &String {
        self.expect_identifier().expect_string()
    }

    pub fn expect_account_id(&self) -> &AccountId {
        self.expect_identifier().expect_account_id()
    }

    pub fn expect_u64(&self) -> u64 {
        self.expect_identifier().expect_u64()
    }
}

pub type FieldId = String;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum Identifier {
    AccountId(AccountId),
    String(String),
    U64(u64),
}

impl Identifier {
    pub fn is_string(&self) -> bool {
        matches!(self, Identifier::String(_))
    }

    pub fn is_account_id(&self) -> bool {
        matches!(self, Identifier::AccountId(_))
    }

    pub fn is_u64(&self) -> bool {
        matches!(self, Identifier::U64(_))
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Identifier::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_account_id(&self) -> Option<&AccountId> {
        match self {
            Identifier::AccountId(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Identifier::U64(value) => Some(*value),
            _ => None,
        }
    }

    // If you want to unwrap with custom error messages
    pub fn expect_string(&self) -> &String {
        match self {
            Identifier::String(value) => value,
            _ => panic!("Expected String identifier"),
        }
    }

    pub fn expect_account_id(&self) -> &AccountId {
        match self {
            Identifier::AccountId(value) => value,
            _ => panic!("Expected AccountId identifier"),
        }
    }

    pub fn expect_u64(&self) -> u64 {
        match self {
            Identifier::U64(value) => *value,
            _ => panic!("Expected U64 identifier"),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct GovernedValue {
    pub id: GovernedValueId,
    pub video_id: VideoId,
    pub dao_id: DaoId,
    pub operator_id: Option<ContractId>,
    pub method: VoteMethod,
}

