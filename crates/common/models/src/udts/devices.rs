use charybdis::{
    macros::charybdis_udt_model,
    types::{Map, SmallInt, Text, TinyInt},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[charybdis_udt_model(type_name=servoudt)]
pub struct ServoUDT {
    pub pin: TinyInt,
    pub angle_min: SmallInt,
    pub angle_max: SmallInt,
    pub angle_default: SmallInt,
    pub position: Text,
}

#[derive(Serialize, Deserialize)]
#[charybdis_udt_model(type_name=motorudt)]
pub struct MotorUDT {
    pub pin: TinyInt,
    pub speed_min: SmallInt,
    pub speed_max: SmallInt,
    pub speed_default: SmallInt,
    pub position: Text,
}

/// Sensor is a base generic struct that represents a sensor. Any sensor can be represented by this struct.
/// Set the correct values for the sensor in the values field.
#[derive(Serialize, Deserialize)]
#[charybdis_udt_model(type_name=sensorudt)]
pub struct SensorUDT {
    pub name: Text,
    pub description: Text,
    pub values: Map<Text, Text>,
}
