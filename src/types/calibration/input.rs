use crate::*;

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub struct CalibrationInput {
//     // Calibration fields
//     pub value_id: ValueId,
//     pub dao_id: DaoId,
//     pub group_id: GroupId,
//     pub cooldown_period: u64,
//     pub adjustment_factor: AdjustmentFactor,
//     // Video fields
//     pub title: String,        
//     pub description: Option<String>,           
//     pub video: VideoHash,                
//     pub image: ImageHash,             
//     pub location: Option<String>,
// }

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CalibrationInput {
    // Calibration fields
    pub reference: CalibrationReference,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub kind: CalibrationKind,
    pub cooldown_period: u64,
    pub adjustment_factor: AdjustmentFactor,
    // Video fields
    pub title: String,        
    pub description: Option<String>,           
    pub video: VideoHash,                
    pub image: ImageHash,             
    pub location: Option<String>,
}