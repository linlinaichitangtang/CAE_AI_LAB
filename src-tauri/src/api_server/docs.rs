use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "CAELab REST API",
        version = "0.6.0",
        description = "REST API for CAELab - Research & Engineering Platform"
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Projects", description = "Project management"),
        (name = "Files", description = "File and note management"),
        (name = "Simulations", description = "CAE simulation management"),
        (name = "Mesh", description = "Mesh generation"),
        (name = "Users", description = "User profile management"),
        (name = "Collaboration", description = "Project sharing and collaboration"),
        (name = "API Keys", description = "API key management"),
    ),
    paths()
)]
struct ApiDoc;

/// Configure Swagger UI and OpenAPI docs routes
pub fn configure() -> Router<super::AppState> {
    let openapi = ApiDoc::openapi();
    Router::new()
        .merge(SwaggerUi::new("/docs/swagger-ui")
            .url("/docs/openapi.json", openapi))
}
