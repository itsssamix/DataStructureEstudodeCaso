use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;

pub struct ProductGraph {
    graph: Graph<Product, Relation>,  // Grafo: nós = produtos, arestas = relações
    node_indices: HashMap<String, NodeIndex>,  // Mapeia ID do produto → nó no grafo
}

impl ProductGraph {
    pub fn new() -> Self {
        ProductGraph {
            graph: Graph::new(),
            node_indices: HashMap::new(),
        }
    }

    // Adiciona um produto ao grafo
    pub fn add_product(&mut self, product: Product) -> NodeIndex {
        let node_index = self.graph.add_node(product.clone());
        self.node_indices.insert(product.id.clone(), node_index);
        node_index
    }

    // Cria uma relação entre dois produtos
    pub fn add_relation(&mut self, product_id1: &str, product_id2: &str, relation: Relation) {
        if let (Some(&node1), Some(&node2)) = (
            self.node_indices.get(product_id1),
            self.node_indices.get(product_id2),
        ) {
            self.graph.add_edge(node1, node2, relation);
        }
    }

    // Busca recomendações usando DFS (Depth-First Search)
    pub fn get_recommendations(&self, product_id: &str, depth: usize) -> Vec<Product> {
        let mut recommendations = Vec::new();
        
        if let Some(&start_node) = self.node_indices.get(product_id) {
            let mut dfs = Dfs::new(&self.graph, start_node);
            
            while let Some(node) = dfs.next(&self.graph) {
                if node != start_node {  // Evita recomendar o próprio produto
                    if let Some(product) = self.graph.node_weight(node) {
                        recommendations.push(product.clone());
                    }
                }
                
                if recommendations.len() >= depth {
                    break;
                }
            }
        }
        
        recommendations
    }
}