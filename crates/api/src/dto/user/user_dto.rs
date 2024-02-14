use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::user::user_roles::UserRoles;

#[derive(Deserialize, Serialize)]
pub struct RequestFindUserDTO {
    pub email: String,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseUserDTO {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub roles: Vec<UserRoles>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
