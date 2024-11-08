use crate::*;

pub fn parse_account_id(id_str: String) -> AccountId {
    id_str.parse::<AccountId>().unwrap_or_else(|_| "default.near".parse().unwrap())
}