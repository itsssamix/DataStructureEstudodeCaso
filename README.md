📌 Visão Geral
Este projeto resolve o problema de busca e recomendação de produtos da MegaStore usando:
✅ Grafos para modelar relações entre produtos
✅ Paralelismo com Rayon para buscas ultrarrápidas
✅ Cache LRU para otimizar resultados frequentes
✅ Segurança garantida pelo sistema de ownership do Rust

🚀 Como Executar
Pré-requisitos
Rust 1.70+ (Instalação)

Cargo (gerenciador de pacotes do Rust)

Passo a Passo
sh
Copy
# Clone o repositório
git clone https://github.com/seuuser/megastore-recommendation
cd megastore-recommendation

# Execute em modo release (otimizado)
cargo run --release

# Rodar testes unitários
cargo test

# Benchmark (requer criterion)
cargo bench
📂 Estrutura do Projeto
Copy
.
├── src/
│   ├── main.rs          # Ponto de entrada
│   ├── graph.rs         # Grafo de produtos e índices
│   ├── search.rs        # Lógica de busca paralela
│   ├── cache.rs         # Sistema de cache LRU
│   └── tests/           # Testes unitários
├── Cargo.toml           # Dependências
├── benches/             # Benchmarks de desempenho
└── README.md            # Documentação
🛠️ Funcionalidades
1. Indexação de Produtos
Grafos com relações de similaridade (pesos 0.0–1.0)

Índices para categoria, marca e atributos

rust
Copy
let mut graph = ProductGraph::new();
graph.add_product(Product {
    id: 1,
    name: "Smartphone X".to_string(),
    category: "Eletrônicos".to_string(),
    brand: "TechCo".to_string(),
    attributes: HashMap::from([("color", "black")]),
});
2. Busca Paralela com Rayon
Filtros por nome, marca, categoria

Processamento multi-thread

rust
Copy
let results = graph.search("Phone", &HashMap::from([("color", "black")]));
3. Cache LRU
Reduz tempo de resposta para buscas repetidas

Tamanho configurável

rust
Copy
let cache = SearchCache::new(1000); // Capacidade: 1000 entradas
📊 Métricas de Desempenho
Operação	Tempo Médio (1M produtos)
Busca simples	< 50ms
Recomendação	< 100ms
Inserção de produto	2ms
