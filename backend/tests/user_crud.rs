use std::env;
use axum::http::{Request, StatusCode};
use axum::body::Body;
use axum::body::to_bytes;
use axum::Router;
use dotenvy::dotenv;
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};
use tower::ServiceExt;
use sqlx::migrate::Migrator;
use sqlx::PgPool;

use gaia::router::create_router;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

async fn setup_test_app() -> Router {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("`DATABASE_URL` should be set");
    let pool = PgPool::connect(&database_url).await.unwrap();
    MIGRATOR.undo(&pool, 100).await.unwrap();
    MIGRATOR.run(&pool).await.expect("Failed running migrations");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    create_router(pool.clone().into())
        .layer(cors)
}

#[tokio::test]
async fn test_post_user() {
    let app = setup_test_app().await;

    let payload = json!({
        "username": "posttestuser",
        "password": "testpass",
        "email": "posttest@example.com"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/users")
                .header("Content-Type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED)
}
