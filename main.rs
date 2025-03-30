mod graph;
mod web;
mod models;

#[actix_web::main]  // Macro para runtime assíncrono
async fn main() -> std::io::Result<()> {
    web::run_server().await  // Inicia o servidor
}