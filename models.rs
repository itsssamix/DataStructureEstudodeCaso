use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,       // ID único (ex: "prod-123")
    pub name: String,     // Nome do produto
    pub category: String, // Categoria (ex: "eletrônicos")
    // Outros campos: preço, descrição, etc.
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Relation {
    BoughtTogether,     // Comprados juntos (frequência)
    SimilarCategory,    // Mesma categoria
    ViewedTogether,     // Visualizados juntos
}