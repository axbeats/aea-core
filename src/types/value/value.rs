use crate::*;

pub type ValueId = u64;
pub type ValueName = String;
pub type SubValueName = String;




// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub enum Value {
//     AccountId(AccountId),
//     Group(HashSet<AccountId>),
//     Amount(YoctoNumber),
//     Percentage(YoctoNumber),
//     Distribution(Distribution),
//     Weight(Weight),
//     Bool(bool),
//     String(String),
//     VideoId(VideoId),
// }

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub enum Value {
//     Int8(i8),
//     Int16(i16),
//     Int32(i32),
//     Int64(i64),
//     Int128(i128),
//     UInt8(u8),
//     UInt16(u16),
//     UInt32(u32),
//     UInt64(u64),
//     UInt128(u128),
//     Float32(f32),
//     Float64(f64),
//     String(String),
//     Bool(bool),
//     Bytes(Vec<u8>),
//     AccountId(AccountId),
// }

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub struct GovernedValue {
//     pub id: ValueId,
//     pub parent_collection_id: Option<CollectionId>,
//     pub child_collection_id: Option<CollectionId>,
//     pub dao_id: DaoId,
//     pub operator_id: ContractId,
//     pub name: String,
//     pub value: Value,
//     pub vote_method: VoteMethod,
// }

// pub type CollectionId = u64; 

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub struct Collection {
//     pub id: CollectionId,
//     pub dao_id: DaoId,
//     pub values: HashSet<ValueId>,
//     pub vote_method: VoteMethod,
// }

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub struct GovernedValue {
//     pub id: ValueId,
//     pub video_id: VideoId,
//     pub dao_id: DaoId,
//     pub operator_id: ContractId,
//     pub value: Value,
//     pub vote_method: VoteMethod,
// }

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub struct Value2 {
//     pub id: ValueId,
//     pub video_id: VideoId,
//     pub type_id: TypeId,
//     pub dao_id: DaoId,
//     pub operator_id: ContractId,
//     pub vote_method: VoteMethod,
// }


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Value {
    pub id: ValueId,
    pub name: ValueName,
    pub description: String,
    pub dao_id: DaoId,
    pub structure: ValueStructure,
    pub vote_method: VoteMethod, // Rename to change_method - Oct 2 2024
    pub calling_contract: ContractId,
}

// impl Value {
//     pub fn from_input(input: ValueInput, id: ValueId, choice_id: Option<ChoiceId>) -> Self {
//         // Match the vote_method_input and ensure ChoiceId is provided if necessary
//         let vote_method = match input.vote_method_input {
//             VoteMethodInput::Proposal => VoteMethod::Proposal,
//             // VoteMethodInput::Choice(_, _, _) => {
//                 VoteMethodInput::Choice => {
//                 if let Some(choice_id) = choice_id {
//                     VoteMethod::Choice(choice_id)
//                 } else {
//                     panic!("ERR_CHOICE_ID: ChoiceId is required for VoteMethod::Choice");
//                 }
//             },
//         };

//         // Construct the Value object based on the input fields
//         Value {
//             id,                          
//             name: input.name,     
//             description: input.description,       
//             dao_id: env::predecessor_account_id(),
//             structure: input.structure,
//             vote_method,      
//             calling_contract: input.calling_contract,
//         }
//     }
// }
