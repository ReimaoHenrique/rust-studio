use axum::Router;
use tower_http::services::ServeDir;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Configura o router para servir arquivos estáticos do frontend/dist
    let app = Router::new().fallback_service(ServeDir::new("src/frontend/dist"));

    // Endereço do servidor
    let addr = SocketAddr::from(([0, 0, 0, 0], 5555));
    println!("Servidor rodando em http://{}", addr);

    // Rodar o servidor usando tokio e axum
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
