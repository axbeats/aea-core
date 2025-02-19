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

pub fn hash_rule_id(rule_id: &RuleId) -> CryptoHash {
    let bytes = rule_id.to_le_bytes();
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(&bytes));
    hash
}

pub fn hash_flag_id(flag_id: &FlagId) -> CryptoHash {
    let bytes = flag_id.to_le_bytes();
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(&bytes));
    hash
}

pub fn hash_review_id(review_id: &ReviewId) -> CryptoHash {
    let bytes = review_id.to_le_bytes();
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

pub fn hash_proposal_id(proposal_id: &ProposalId) -> CryptoHash {
    let bytes = proposal_id.to_le_bytes();
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

pub fn hash_choice_id(choice_id: &ChoiceId) -> CryptoHash {
    let bytes = choice_id.to_le_bytes();
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(&bytes));
    hash
}

pub fn hash_choice_account_id(choice_id: &ChoiceId, account_id: &AccountId) -> CryptoHash {
    // Create a unique combination of ChoiceId and AccountId
    let mut data = choice_id.to_le_bytes().to_vec();
    data.extend(account_id.as_bytes());

    // Initialize a default CryptoHash
    let mut hash = CryptoHash::default();

    // Compute the SHA-256 hash of the combined data
    hash.copy_from_slice(&env::sha256(&data));

    hash
}
