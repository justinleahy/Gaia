use axum::{Json, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::Utc;
use utoipa::{OpenApi, ToSchema};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;
use serde::Serialize;

#[derive(Serialize, ToSchema)]
struct Health {
    #[serde(rename = "CurrentTime")]
    current_time: String,
}

#[utoipa::path(
    get,
    path = "/api/health",
    tag = "Health",
    responses(
        (status = 200, description = "Returns server's current time and status for basic availability monitoring.", body = Health)
    )
)]
pub async fn health() -> impl IntoResponse {
    let health = Health {
        current_time: Utc::now().to_rfc3339()
    };

    (StatusCode::OK, Json(health))
}

pub fn create_router() -> Router {
    #[derive(OpenApi)]
    #[openapi()]
    struct ApiDoc;

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(health))
        .split_for_parts();

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()));

    return router;
}