use charybdis::{
    macros::charybdis_model,
    types::{Frozen, List, Text, Timestamp},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::udts::{motor_udt::MotorUDT, servo_udt::ServoUDT};

#[charybdis_model(
    table_name = aircraft,
    partition_keys = [id],
    clustering_keys = [user_id, created_at],
)]
#[derive(Deserialize, Serialize)]
pub struct AircraftModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub aircraft_type: Text,
    pub name: Text,
    pub description: Text,
    pub servos: Frozen<List<Frozen<ServoUDT>>>,
    pub motors: Frozen<List<Frozen<MotorUDT>>>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
