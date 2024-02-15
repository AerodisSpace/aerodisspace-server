use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// REGISTER -------------------------------------------------------------------------
#[derive(Deserialize)]
pub struct UserSignupRequestDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserSignupResponseDTO {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub roles: HashSet<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
// LOGIN -------------------------------------------------------------------------
#[derive(Deserialize, Serialize)]
pub struct UserLoginRequestDTO {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserLoginResponseDTO {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub roles: HashSet<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub token: String,
}
// UPDATE USER -------------------------------------------------------------------------
