use axum::{
    body::Body,
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use super::handlers::AuthUser;

/// JWT authentication middleware
/// Extracts the Bearer token from the Authorization header and validates it.
/// On success, inserts the authenticated user_id into request extensions.
pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Response {
    // Extract Authorization header
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    let token = match auth_header {
        Some(h) if h.starts_with("Bearer ") => &h[7..],
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({
                    "success": false,
                    "error": "Missing or invalid Authorization header. Use: Bearer <token>"
                })),
            )
                .into_response();
        }
    };

    // Verify JWT token
    match crate::commands::auth::verify_token_public(token, "access") {
        Ok(user_id) => {
            req.extensions_mut().insert(AuthUser { user_id });
            next.run(req).await
        }
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            axum::Json(serde_json::json!({
                "success": false,
                "error": e
            })),
        )
            .into_response(),
    }
}
