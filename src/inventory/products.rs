use fake::Dummy;

/// A category of product that our businees sells
#[derive(Debug, Dummy)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}

/// A concrate item stock within our pronject_structure
#[derive(Debug, Dummy)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32,
}

impl Item {
    /// Create a new item
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {
        super::talk_to_manager();
        Self {
            name,
            category,
            quantity,
        }
    }
}