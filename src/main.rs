use axum::{
    http::StatusCode,
    routing::{get, post},
    Json,
    Router,
};

use serde_json::Value;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health_check", get(health_check))
        .route("/webhook", post(webhook_handler));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("Listening for Vault tx on http://127.0.0.1:3000");
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

async fn webhook_handler(Json(payload): Json<Value>) -> StatusCode {
    println!("Transaction received:\n{:#}\n", payload);
    StatusCode::OK
}
