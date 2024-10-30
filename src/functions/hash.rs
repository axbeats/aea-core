use near_sdk::CryptoHash;

use crate::*;

pub fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the account ID and return it
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

pub fn hash_video_id(video_id: &VideoId) -> CryptoHash {
    let bytes = video_id.to_le_bytes();
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(&bytes));
    hash
}

pub fn hash_law_id(law_id: &LawId) -> CryptoHash {
    let bytes = law_id.to_le_bytes();
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(&bytes));
    hash
}

pub fn hash_indictment_id(indictment_id: &IndictmentId) -> CryptoHash {
    let bytes = indictment_id.to_le_bytes();
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(&bytes));
    hash
}

pub fn hash_day(day: &u64) -> CryptoHash {
    let bytes = day.to_le_bytes();
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(&bytes));
    hash
}

pub fn hash_comment_id(day: &u64) -> CryptoHash {
    let bytes = day.to_le_bytes();
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(&bytes));
    hash
}

pub fn hash_calibration_id(calibration_id: &CalibrationId) -> CryptoHash {
    let bytes = calibration_id.to_le_bytes();
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(&bytes));
    hash
}