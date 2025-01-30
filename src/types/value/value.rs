use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub id: ValueId,
    pub video_id: VideoId,
    pub dao_id: DaoId,
    pub operator_id: Option<ContractId>,
    pub method: VoteMethod,
}

