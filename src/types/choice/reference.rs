use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ChoiceReference {
    pub contract_id: ContractId,
    pub identifier: ChoiceIdentifier,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ChoiceIdentifier {
    AccountId(AccountId),
    Bio(AccountId),
    GroupId(GroupId),
}