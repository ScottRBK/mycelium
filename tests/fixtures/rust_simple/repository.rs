use std::collections::HashMap;
use crate::model::Item;

pub struct InMemoryRepository {
    items: HashMap<u32, Item>,
    counter: u32,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        InMemoryRepository {
            items: HashMap::new(),
            counter: 0,
        }
    }

    pub fn find_by_id(&self, id: u32) -> Option<&Item> {
        self.items.get(&id)
    }

    pub fn find_all(&self) -> Vec<&Item> {
        self.items.values().collect()
    }

    pub fn save(&mut self, item: Item) {
        self.items.insert(item.id, item);
    }

    pub fn delete(&mut self, id: u32) -> bool {
        self.items.remove(&id).is_some()
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn exists(&self, id: u32) -> bool {
        self.items.contains_key(&id)
    }
}
