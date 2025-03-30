use actix_web::{get, web, App, HttpServer, Responder};
use std::sync::Mutex;

// Estado compartilhado (thread-safe)
pub struct AppState {
    product_graph: Mutex<ProductGraph>,  // Mutex evita race conditions
}

// Rota: /recommendations/{product_id}
#[get("/recommendations/{product_id}")]
async fn get_recommendations(
    data: web::Data<AppState>,  // Acesso ao estado
    product_id: web::Path<String>,  // Parâmetro da URL
) -> impl Responder {
    let graph = data.product_graph.lock().unwrap();  // Bloqueia o Mutex
    let recommendations = graph.get_recommendations(&product_id, 5);
    web::Json(recommendations)  // Retorna JSON
}

// Inicia o servidor
pub async fn run_server() -> std::io::Result<()> {
    let product_graph = ProductGraph::new();
    // Adiciona produtos e relações aqui...

    let app_data = web::Data::new(AppState {
        product_graph: Mutex::new(product_graph),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())  // Compartilha estado entre threads
            .service(get_recommendations)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}