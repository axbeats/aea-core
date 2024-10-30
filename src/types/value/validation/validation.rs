use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum ValidationType {
    AccountId(AccountIdValidation),
    Int64(NumberValidation<i64>),
    UInt8(NumberValidation<u8>),
    UInt64(NumberValidation<u64>),
    UInt128(NumberValidation<u128>),
    String(StringValidation),
    Boolean(BooleanValidation),
    Bytes(BytesValidation),
    Video(MediaValidation),
    Image(MediaValidation),
    Audio(MediaValidation),
    Location(LocationValidation),
    Date(DateValidation),
    Time(TimeValidation),
}