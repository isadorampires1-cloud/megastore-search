#[derive(Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub brand: String,
}

impl Product {
    pub fn new(id: u32, name: &str, category: &str, brand: &str) -> Self {
        Product {
            id,
            name: name.to_string(),
            category: category.to_string(),
            brand: brand.to_string(),
        }
    }
}
