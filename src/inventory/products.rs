#[derive(Debug)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}
#[derive(Debug)]

pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32,
}

impl Item {
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {
        super::talk_to_manager();
        Self {
            name,
            category,
            quantity,
        }
    }
}