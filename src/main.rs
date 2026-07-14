use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(hello_world))
    .route("/health_check", get(health_check))
    .route("/echo", post(echo));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await
    .expect("Failed to bind to port 3000");

    axum::serve(listener, app)
    .await
    .expect("Server failed to start");
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn health_check() -> &'static str {
    "OK"
}

async fn echo(body: String) -> String {
    format!("You sent: {}", body)
}