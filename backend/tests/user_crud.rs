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
async fn test_put_user() {
    let app = setup_test_app().await;

    let payload = json!({
        "username": "puttestuser",
        "password": "testpass",
        "email": "puttest@example.com",
        "first_name": "Put",
        "last_name": "Test"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/v1/users")
                .header("Content-Type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED)
}

#[tokio::test]
async fn test_get_user() {
    let app = setup_test_app().await;

    let payload = json!({
        "username": "gettestuser",
        "password": "testpass",
        "email": "gettest@example.com",
        "first_name": "Get",
        "last_name": "Test"
    });

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/v1/users")
                .header("Content-Type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(post_response.status(), StatusCode::CREATED);

    let body = to_bytes(post_response.into_body(), 1000).await.unwrap();
    let response_data: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let user_id = response_data["id"].as_str().unwrap();

    let get_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/users/{}", user_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    let body = to_bytes(get_response.into_body(), 1000).await.unwrap();
    let user: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(user["id"], user_id);
    assert_eq!(user["username"], "gettestuser");
    assert_eq!(user["email"], "gettest@example.com");
    assert!(!user.as_object().unwrap().contains_key("password"))
}

#[tokio::test]
async fn test_post_user() {
    let app = setup_test_app().await;

    let put_payload = json!({
        "username": "posttestuser",
        "password": "testpass",
        "email": "posttest@example.com",
        "first_name": "Post",
        "last_name": "Post"
    });

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/v1/users")
                .header("Content-Type", "application/json")
                .body(Body::from(put_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(post_response.status(), StatusCode::CREATED);

    let body = to_bytes(post_response.into_body(), 1000).await.unwrap();
    let response_data: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let user_id = response_data["id"].as_str().unwrap();



    let get_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/users/{}", user_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    let body = to_bytes(get_response.into_body(), 1000).await.unwrap();
    let user: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(user["id"], user_id);
    assert_eq!(user["username"], "gettestuser");
    assert_eq!(user["email"], "gettest@example.com");
    assert!(!user.as_object().unwrap().contains_key("password"))
}