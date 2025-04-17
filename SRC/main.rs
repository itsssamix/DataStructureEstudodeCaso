mod graph;
mod search;
mod cache;

use graph::{Product, ProductGraph};
use search::SearchCache;
use std::collections::HashMap;

fn main() {
    let mut graph = ProductGraph::new();
    graph.add_product(Product {
        id: 1,
        name: "Smartphone X".to_string(),
        category: "Eletrônicos".to_string(),
        brand: "TechCo".to_string(),
        attributes: HashMap::from([("color".to_string(), "black".to_string())]),
    });

    let cache = SearchCache::new(1000);
    let results = cache.get_or_insert(
        "Smartphone".to_string(),
        || graph.search("Smartphone", &HashMap::new())
    );

    println!("Resultados: {:?}", results);
}