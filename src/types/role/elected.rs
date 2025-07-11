use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ElectedRole {
    pub members: HashSet<AccountId>,
    pub choice_id: ChoiceId,
}