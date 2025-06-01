use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String
}

#[derive(Deserialize, ToSchema)]
pub struct PutUserRequest {
    pub username: String,
    pub password: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String
}

#[derive(Deserialize, ToSchema)]
pub struct PostUserRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>
}
