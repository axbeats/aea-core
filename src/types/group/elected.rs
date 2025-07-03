use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ElectedGroup {
    pub members: HashSet<AccountId>,
    pub choice_id: ChoiceId,
}