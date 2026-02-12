mod service;
mod model;
mod error;
mod repository;

use service::DataService;
use model::Item;
use error::AppError;

pub struct Handler {
    svc: DataService,
}

impl Handler {
    pub fn new() -> Self {
        Handler {
            svc: DataService::new(),
        }
    }

    pub fn handle_get(&self, id: u32) -> Result<Option<String>, AppError> {
        Ok(self.svc.get_item(id))
    }

    pub fn handle_create(&mut self, name: &str) -> Result<u32, AppError> {
        if name.is_empty() {
            return Err(AppError::validation("Name cannot be empty"));
        }
        Ok(self.svc.create_item(name))
    }

    pub fn handle_delete(&mut self, id: u32) -> Result<bool, AppError> {
        Ok(self.svc.delete_item(id))
    }

    pub fn handle_list(&self) -> Vec<Item> {
        self.svc.list_items()
    }
}

fn main() {
    let mut handler = Handler::new();
    let id = handler.handle_create("test").unwrap();
    println!("Created: {}", id);
    println!("Get: {:?}", handler.handle_get(id));
    println!("All: {:?}", handler.handle_list());
}
