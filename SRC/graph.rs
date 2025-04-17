use petgraph::graph::{Graph, NodeIndex};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub category: String,
    pub brand: String,
    pub attributes: HashMap<String, String>,
}

pub struct ProductGraph {
    pub graph: Graph<Product, f32>, // Arestas = similaridade (0.0 a 1.0)
    pub index: HashMap<u64, NodeIndex>, // Mapeia ID → Nó
    pub category_index: HashMap<String, Vec<NodeIndex>>,
    pub brand_index: HashMap<String, Vec<NodeIndex>>,
}

impl ProductGraph {
    pub fn new() -> Self {
        ProductGraph {
            graph: Graph::new(),
            index: HashMap::new(),
            category_index: HashMap::new(),
            brand_index: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        let node = self.graph.add_node(product.clone());
        self.index.insert(product.id, node);
        self.category_index.entry(product.category.clone()).or_default().push(node);
        self.brand_index.entry(product.brand.clone()).or_default().push(node);
    }
}