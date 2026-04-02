use axum::{
    routing::{get, post, put, delete},
    Router,
};

use super::handlers;
use super::middleware::auth_middleware;
use super::AppState;

/// Configure all REST API routes
pub fn configure() -> Router<AppState> {
    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/auth/login", post(handlers::login))
        .route("/auth/register", post(handlers::register))
        .route("/auth/refresh", post(handlers::refresh_token))
        .route("/health", get(handlers::health_check));

    // Protected routes (auth required)
    let protected_routes = Router::new()
        // Projects
        .route("/projects", get(handlers::list_projects))
        .route("/projects", post(handlers::create_project))
        .route("/projects/{id}", get(handlers::get_project))
        .route("/projects/{id}", put(handlers::update_project))
        .route("/projects/{id}", delete(handlers::delete_project))
        // Files
        .route("/projects/{id}/files", get(handlers::list_files))
        .route("/projects/{id}/files", post(handlers::create_file))
        .route("/files/{id}", get(handlers::get_file))
        .route("/files/{id}", put(handlers::update_file))
        .route("/files/{id}", delete(handlers::delete_file))
        // Simulations
        .route("/simulations", post(handlers::run_simulation))
        .route("/simulations/{id}", get(handlers::get_simulation_status))
        .route("/simulations/{id}/result", get(handlers::get_simulation_result))
        // Mesh
        .route("/mesh/generate", post(handlers::generate_mesh))
        // Users
        .route("/users/me", get(handlers::get_profile))
        .route("/users/me", put(handlers::update_profile))
        // Collaboration
        .route("/projects/{id}/share", post(handlers::share_project))
        .route("/projects/{id}/shares", get(handlers::list_shares))
        // API Keys
        .route("/api-keys", get(handlers::list_api_keys))
        .route("/api-keys", post(handlers::create_api_key))
        .route("/api-keys/{id}", delete(handlers::delete_api_key))
        .route_layer(axum::middleware::from_fn(auth_middleware));

    public_routes.merge(protected_routes)
}
