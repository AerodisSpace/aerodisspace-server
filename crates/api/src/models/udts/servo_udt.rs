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
