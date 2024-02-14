use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::user::user_roles::UserRoles;

#[derive(Deserialize)]
pub struct RequestRegisUserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RequestLoginUserDTO {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseLoginUserDTO {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub roles: Vec<UserRoles>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub token: String,
}

