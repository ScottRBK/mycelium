#[derive(Debug, Clone)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub active: bool,
}

impl Item {
    pub fn new(id: u32, name: String) -> Self {
        Item {
            id,
            name,
            category: String::from("default"),
            active: true,
        }
    }

    pub fn with_category(mut self, category: &str) -> Self {
        self.category = category.to_string();
        self
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

#[derive(Debug, Clone)]
pub struct ItemFilter {
    pub category: Option<String>,
    pub active_only: bool,
}

impl Default for ItemFilter {
    fn default() -> Self {
        ItemFilter {
            category: None,
            active_only: true,
        }
    }
}
