use axum::{
    Router
};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;

#[utoipa::path(
    get,
    path = "/home",
    tag = "Home",
    responses(
        (status = 200, description = "Displays the home page")
    )
)]
async fn home() -> impl IntoResponse {
    (StatusCode::OK, "home")
}

pub fn create_router() -> Router {
    #[derive(OpenApi)]
    #[openapi()]
    struct ApiDoc;

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(home))
        .split_for_parts();

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .route("/", get(home));

    return router;
}