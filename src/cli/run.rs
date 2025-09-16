use axum::Router;
use tower_http::services::ServeDir;
use std::net::SocketAddr;

pub async fn run_server() {
    let app = Router::new().fallback_service(ServeDir::new("src/frontend/dist"));
    let addr = SocketAddr::from(([0, 0, 0, 0], 5555));
    println!("Rust-Studio rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Falha ao bindar o endereço");
    axum::serve(listener, app)
        .await
        .expect("Falha ao rodar o servidor");
}
