use charybdis::{
    macros::charybdis_udt_model,
    types::{Map, SmallInt, Text, TinyInt},
};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[charybdis_udt_model(type_name=motorudt)]
pub struct MotorUDT {
    pub pin: TinyInt,
    pub speed_min: SmallInt,
    pub speed_max: SmallInt,
    pub speed_default: SmallInt,
    pub position: Text,
}
