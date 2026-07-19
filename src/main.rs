use axum::{
    Json, Router,
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
};
use serde::Deserialize;

#[derive(Clone)]
struct AppState {
    webhook_secret: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HeliusTransaction {
    signature: String,
    slot: u64,
    timestamp: i64,
    fee: u64,
    fee_payer: String,
    source: String,
    description: String,
    #[serde(rename = "type")]
    tx_type: String,
}

#[tokio::main]
async fn main() {
    let webhook_secret = std::env::var("WEBHOOK_SECRET").expect("WEBHOOK_SECRET should be defined");
    let state = AppState { webhook_secret };

    let helius_webhook = Router::new()
        .route("/webhook", post(webhook_handler))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware));

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health_check", get(health_check))
        .merge(helius_webhook);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
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

async fn auth_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let received = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    match received {
        Some(value) if value == state.webhook_secret => Ok(next.run(request).await),
        _ => {
            eprintln!("Rejected : Invalid auth header");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

async fn webhook_handler(
    Json(transactions): Json<Vec<HeliusTransaction>>
) -> StatusCode {
    for tx in &transactions {
        println!(
            "{} | slot {} | {} lamports | type: {}\n  {}",
            tx.signature, tx.slot, tx.fee, tx.tx_type, tx.description
        );
    }
    StatusCode::OK
}
