use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct VideoInteraction {
    pub viewed: bool,
    pub liked: bool,
    pub favourited: bool,
    pub commented: bool,
    pub shared: bool,
    pub collaborated: bool,
}