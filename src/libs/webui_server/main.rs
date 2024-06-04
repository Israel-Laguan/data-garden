use axum::{
    routing::{get, post},
    Router, Server,
};
use std::net::TcpListener;
use tower_http::services::ServeDir;

pub async fn start_web_server() -> Result<(), std::io::Error> {
    let default_port = 3000;
    let address = format!("127.0.0.1:{}", default_port);

    // Verify if the default port is available
    if TcpListener::bind(&address).is_err() {
        return Err(std::io::Error::new(std::io::ErrorKind::AddrInUse, "Port is already in use"));
    }

    // Define the router with routes
    let app = Router::new()
        .route("/", get(serve_html_content))
        .route("/api/data", post(handle_data_endpoint));

    // Start the server
    println!("Starting server at http://{}", address);
    Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

async fn serve_html_content() -> Result<String, std::io::Error> {
    ServeDir::new("static").call(()).await
}

async fn handle_data_endpoint() -> Result<String, std::io::Error> {
    // Implement endpoint logic
    Ok("Data processed".to_string())
}
