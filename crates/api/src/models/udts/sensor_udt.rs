use charybdis::{
    macros::charybdis_udt_model,
    types::{Map, SmallInt, Text, TinyInt},
};
use serde::{Deserialize, Serialize};



/// Sensor is a base generic struct that represents a sensor. Any sensor can be represented by this struct.
/// Set the correct values for the sensor in the values field.
#[derive(Serialize, Deserialize)]
#[charybdis_udt_model(type_name=sensorudt)]
pub struct SensorUDT {
    pub name: Text,
    pub description: Text,
    pub values: Map<Text, Text>,
}
