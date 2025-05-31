use std::sync::Arc;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;
use crate::models::{GetUser, PostUser, User};


pub(crate) fn router(pool: Arc<PgPool>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(post_user, get_user))
        .with_state(pool)
}

#[utoipa::path(
    post,
    path = "",
    tag = "User",
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 500, description = "User failed to be created")
    )
)]
async fn post_user(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<PostUser>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(payload.password.as_bytes(), &salt).unwrap().to_string();

    let user = sqlx::query_as!(
        User,
        r#"INSERT INTO users (id, username, password, email)
           VALUES ($1, $2, $3, $4)
           RETURNING id, username, password, email"#,
        Uuid::new_v4(),
        payload.username,
        password_hash,
        payload.email
    )
        .fetch_one(&*pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(user)))
}

#[utoipa::path(
    get,
    path = "",
    tag = "User",
    responses(
        (status = 200, description = "User found successfully", body = User),
        (status = 404, description = "User not found")
    )
)]
async fn get_user(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<GetUser>
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT * FROM users WHERE id = $1"#,
        payload.id
    )
        .fetch_one(&*pool)
        .await
        .map_err(|e| (axum::http::StatusCode::NOT_FOUND, e.to_string()))?;

    Ok((StatusCode::OK, Json(user)))
}