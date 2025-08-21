mod config;
mod handlers;
mod models;
mod products;

use crate::config::Config;
use crate::handlers::*;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    match dotenv::dotenv() {
        Ok(_) => println!("Successfully loaded .env file"),
        Err(_) => println!("No .env file found, using system environment variables"),
    }

    let config = Arc::new(Config::from_env()?);
    let port = config.port;

    let app = Router::new()
        .route("/", get(root))
        .route("/api/list_product", post(list_products))
        .route("/api/request_payment", post(request_payment))
        .route("/api/callback_paytech", post(paytech_callback))
        .layer(CorsLayer::permissive())
        .with_state(config);

    let bind_address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    println!("Server running on http://{}", bind_address);

    axum::serve(listener, app).await?;

    Ok(())
}
