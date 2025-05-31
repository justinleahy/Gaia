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
pub struct PostUser {
    pub username: String,
    pub password: String,
    pub email: String
}

#[derive(Deserialize, ToSchema)]
pub struct GetUser {
    pub id: Uuid
}

#[derive(Deserialize)]
pub struct PatchUser {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>
}
