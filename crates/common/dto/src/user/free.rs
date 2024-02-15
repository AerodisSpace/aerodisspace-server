use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct UserGetOneRequestDTO {
    pub email: Option<String>,
}


#[derive(Deserialize, Serialize)]
pub struct UserGetResponseDTO {
    pub id: Uuid,
    pub username: String,
    pub roles: Vec<String>, // UserRoles (enum)
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
