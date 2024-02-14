use charybdis::{
    macros::charybdis_model,
    types::{Frozen, List, Timestamp},
};
use uuid::Uuid;

use crate::models::udts::sensor_udt::SensorUDT;

/// Table to store the sensors data of the plane.
/// Sensors is a packet of sensor data that is builded by the controller and sent to the server.
/// The sensors are in the plane, and send the data to the controller, that will build this struct and send to the server.
#[charybdis_model(
    table_name = sensor_data,
    partition_keys = [id],
    clustering_keys = [aircraft_id, created_at]
)]
pub struct SensorDataModel {
    pub id: Uuid,
    pub aircraft_id: Uuid,
    pub sensors: Frozen<List<Frozen<SensorUDT>>>,
    pub created_at: Timestamp,
}
