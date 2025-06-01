use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email: String
}

#[derive(Deserialize, ToSchema)]
pub struct PostUserRequest {
    pub username: String,
    pub password: String,
    pub email: String
}

#[derive(Deserialize)]
pub struct PatchUserRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>
}
