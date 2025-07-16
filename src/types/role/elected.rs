use crate::*;
use aea_macros::Generable;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct ElectedRole {
    pub members: HashSet<AccountId>,
    pub choice_id: ChoiceId,
}

impl Default for ElectedRole {
    fn default() -> Self {
        Self {
            members: HashSet::new(),
            choice_id: ChoiceId::default(),
        }
    }
}