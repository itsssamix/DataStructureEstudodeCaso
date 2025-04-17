use rayon::prelude::*;
use super::graph::{ProductGraph, Product};

impl ProductGraph {
    pub fn search(&self, query: &str, filters: &HashMap<String, String>) -> Vec<u64> {
        self.graph.node_indices()
            .par_bridge() // Paraleliza a busca
            .filter_map(|node| {
                let product = &self.graph[node];
                let matches_query = product.name.contains(query);
                let matches_filters = filters.iter()
                    .all(|(key, value)| product.attributes.get(key) == Some(value));
                
                if matches_query && matches_filters {
                    Some(product.id)
                } else {
                    None
                }
            })
            .collect()
    }
}