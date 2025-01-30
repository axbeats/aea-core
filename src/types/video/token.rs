use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct VideoNFT {
    pub id: VideoNftId,
    pub video_id: VideoId,
    pub owner_id: AccountId,
    pub royalties: HashMap<AccountId, PercentageU32>,
    pub approved_account_ids: HashMap<AccountId, ApprovalId>,
    pub last_approval_id: ApprovalId,
    pub issued_at: TimestampNanoSeconds,
}

impl VideoNFT {
    pub fn from_input(id: TokenId, input: VideoNftInput, video_id: VideoId) -> Self {

        // create a royalty map to store in the token
        let mut royalties = HashMap::new();

        // if perpetual royalties were passed into the function: 
        if let Some(input_royalties) = input.royalties {
            //make sure that the length of the perpetual royalties is below 7 since we won't have enough GAS to pay out that many people
            assert!(input_royalties.len() < 7, "ERR_TOO_MANY_ROYALTIES");

            //iterate through the perpetual royalties and insert the account and amount in the royalty map
            for (account, amount) in input_royalties {
                royalties.insert(account, amount);
            }
        }

        VideoNFT {
            id,
            video_id,
            owner_id: env::predecessor_account_id(),
            royalties,
            approved_account_ids: HashMap::new(),
            last_approval_id: 0,
            issued_at: env::block_timestamp(),
        }
    }
}

// #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub struct VideoTokenInput {
//     pub video_id: VideoId,
//     pub royalties: Option<HashMap<AccountId, PercentageU32>>,
// }

// impl VideoToken {
//     pub fn from_input(id: TokenId, input: VideoTokenInput) -> Self {

//         // create a royalty map to store in the token
//         let mut royalties = HashMap::new();

//         // if perpetual royalties were passed into the function: 
//         if let Some(input_royalties) = input.royalties {
//             //make sure that the length of the perpetual royalties is below 7 since we won't have enough GAS to pay out that many people
//             assert!(input_royalties.len() < 7, "ERR_TOO_MANY_ROYALTIES");

//             //iterate through the perpetual royalties and insert the account and amount in the royalty map
//             for (account, amount) in input_royalties {
//                 royalties.insert(account, amount);
//             }
//         }

//         VideoToken {
//             id,
//             video_id: input.video_id,
//             owner_id: env::predecessor_account_id(),
//             royalties,
//             approved_account_ids: HashMap::new(),
//             last_approval_id: 0,
//             issued_at: env::block_timestamp(),
//         }
//     }
// }