#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_product() {
        let mut graph = ProductGraph::new();
        let product = Product {
            id: "1".to_string(),
            name: "Test Product".to_string(),
            category: "Test".to_string(),
        };
        
        graph.add_product(product);
        assert_eq!(graph.graph.node_count(), 1);
    }

    // Mais testes...
}