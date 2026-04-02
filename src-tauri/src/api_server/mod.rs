pub mod routes;
pub mod handlers;
pub mod middleware;
pub mod docs;

use axum::Router;
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;
use crate::db::Database;

/// Application state shared across all API handlers
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Database>>,
}

/// Build the complete API router
pub fn build_router(db: Arc<Mutex<Database>>) -> Router {
    let state = AppState { db };

    let api_routes = routes::configure();

    Router::new()
        .nest("/api/v1", api_routes)
        .merge(docs::configure())
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Start the API server on the specified port
pub async fn start_api_server(
    port: u16,
    db: Arc<Mutex<Database>>,
    shutdown_rx: tokio::sync::watch::Receiver<bool>,
) -> Result<(), String> {
    let app = build_router(db);
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));

    tracing::info!("REST API server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| format!("Failed to bind to {}: {}", addr, e))?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(shutdown_rx))
        .await
        .map_err(|e| format!("API server error: {}", e))?;

    tracing::info!("REST API server stopped");
    Ok(())
}

async fn shutdown_signal(mut rx: tokio::sync::watch::Receiver<bool>) {
    if rx.changed().await.is_ok() && *rx.borrow() {
        tracing::info!("API server shutdown signal received");
    }
}
