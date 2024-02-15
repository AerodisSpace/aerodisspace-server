use charybdis::{
    macros::charybdis_model,
    types::{Set, Text, Timestamp, TinyInt},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[charybdis_model(
    table_name = user,
    partition_keys = [email],
    clustering_keys = [created_at],
)]
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct UserModel {
    pub id: Uuid,
    pub username: Text,
    pub email: Text,
    pub password: Text,
    pub roles: Set<Text>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
