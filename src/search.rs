use std::collections::HashMap;
use crate::product::Product;

pub struct SearchEngine {
    index: HashMap<String, Vec<Product>>,
}

impl SearchEngine {

    pub fn new() -> Self {
        SearchEngine {
            index: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.index
            .entry(product.name.clone())
            .or_insert(Vec::new())
            .push(product);
    }

    pub fn search_by_name(&self, name: &str) -> Vec<Product> {
        self.index
            .iter()
            .filter(|(key, _)| key.contains(name))
            .flat_map(|(_, products)| products.clone())
            .collect()
    }
}
