use std::collections::HashMap;
use crate::model::Item;

pub struct DataService {
    store: HashMap<u32, Item>,
    counter: u32,
}

impl DataService {
    pub fn new() -> Self {
        DataService {
            store: HashMap::new(),
            counter: 0,
        }
    }

    pub fn get_item(&self, id: u32) -> Option<String> {
        self.store.get(&id).map(|item| item.name.clone())
    }

    pub fn create_item(&mut self, name: &str) -> u32 {
        self.counter += 1;
        let item = Item::new(self.counter, name.to_string());
        self.store.insert(self.counter, item);
        self.counter
    }

    pub fn delete_item(&mut self, id: u32) -> bool {
        self.store.remove(&id).is_some()
    }

    pub fn list_items(&self) -> Vec<Item> {
        self.store.values().cloned().collect()
    }

    pub fn update_item(&mut self, id: u32, name: &str) -> Option<Item> {
        if let Some(item) = self.store.get_mut(&id) {
            item.name = name.to_string();
            Some(item.clone())
        } else {
            None
        }
    }

    pub fn count(&self) -> usize {
        self.store.len()
    }
}

pub trait Repository {
    fn find(&self, id: u32) -> Option<String>;
    fn save(&mut self, name: &str) -> u32;
    fn delete(&mut self, id: u32) -> bool;
    fn find_all(&self) -> Vec<Item>;
}
