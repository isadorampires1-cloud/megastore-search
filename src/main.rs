mod product;
mod search;

use product::Product;
use search::SearchEngine;

fn main() {
    let mut engine = SearchEngine::new();

    engine.add_product(Product::new(1, "Notebook Dell", "Eletrônicos", "Dell"));
    engine.add_product(Product::new(2, "Notebook Lenovo", "Eletrônicos", "Lenovo"));
    engine.add_product(Product::new(3, "Smartphone Samsung", "Eletrônicos", "Samsung"));

    let results = engine.search_by_name("Notebook");

    for product in results {
        println!("Produto encontrado: {}", product.name);
    }
}
