use crate::*;

pub trait ProposalHashMapInterface {
    type Object: ProposalHashMap;

    /// Retrieve a reference to the proposal hashmap by identifier
    fn get_hashmap(&self, identifier: ValueIdentifier) -> &Self::Object;

    /// Retrieve a mutable reference to the proposal hashmap by identifier
    fn get_hashmap_mut(&mut self, identifier: ValueIdentifier) -> &mut Self::Object;

    /// Add an item to the hashmap
    fn add_item(
        &mut self, 
        identifier: ValueIdentifier,
        key: <Self::Object as ProposalHashMap>::Key,
        value: <Self::Object as ProposalHashMap>::Value,
    ) -> Result<(), ProposalProtocolError>;

    /// Remove an item from the hashmap
    fn remove_item(
        &mut self, 
        identifier: ValueIdentifier,
        key: <Self::Object as ProposalHashMap>::Key,
    ) -> Result<<Self::Object as ProposalHashMap>::Value, ProposalProtocolError>;
}


pub trait ProposalHashMap {
    type Key: std::cmp::Eq + std::hash::Hash; // Keys must be hashable and comparable
    type Value;

    /// Access the internal `HashMap` mutably
    fn get_map_mut(&mut self) -> &mut HashMap<Self::Key, Self::Value>;

    /// Default implementation for `add_item`
    fn add_item(&mut self, key: Self::Key, item: Self::Value) -> Result<(), ProposalProtocolError> {
        let map = self.get_map_mut();
        if map.contains_key(&key) {
            Err(ProposalProtocolError::DuplicateKey)
        } else {
            map.insert(key, item);
            Ok(())
        }
    }

    /// Default implementation for `remove_item`
    fn remove_item(&mut self, key: Self::Key) -> Result<Self::Value, ProposalProtocolError> {
        let map = self.get_map_mut();
        map.remove(&key).ok_or(ProposalProtocolError::NotFound)
    }
}

#[derive(Debug)]
pub enum ProposalProtocolError {
    DuplicateKey,
    NotFound,
}
