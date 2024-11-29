use crate::*;

pub type MintStreamId = String;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MintStreams {
    pub streams: UnorderedMap<MintStreamId, MintStream>,
}

impl CalibrationDeltaObject for MintStreams {
    type Item = MintStream;

    fn get_item(&self, name: &String) -> Option<Self::Item> {
        self.streams.get(name)
    }

    fn set_item(&mut self, name: &String, item: Self::Item) {
        self.streams.insert(name, &item);
    }

    fn items(&self) -> Vec<Self::Item> {
        self.streams.values().collect()
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MintStream {
    pub id: MintStreamId,
    pub percentage: YoctoPercentage,
    pub interactions: HashSet<MintInteractionKey>,
    pub current_weights: HashMap<AccountId, u128>,
    pub carryover_negative_weights: HashMap<AccountId, u128>,
    pub last_mint_timestamp: TimestampNanoSeconds,
}

impl CalibrationItem for MintStream {
    fn get_percentage(&self) -> YoctoPercentage {
        self.percentage
    }

    fn set_percentage(&mut self, percentage: YoctoPercentage) {
        self.percentage = percentage;
    }
}

impl MintStream {
    pub fn from_input(input: MintStreamInput) -> Self {
        Self { 
            id: input.id, 
            interactions: input.interactions, 
            percentage: input.percentage, 
            current_weights: HashMap::new(), 
            carryover_negative_weights: HashMap::new(), 
            last_mint_timestamp: 0 
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MintStreamInput {
    pub id: MintStreamId,
    pub interactions: HashSet<MintInteractionKey>,
    pub percentage: YoctoPercentage,
}

