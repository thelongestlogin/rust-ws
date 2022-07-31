use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct UserLocation {
    pub altitude:String,
    pub latitude:String,
    pub name:String,
    pub longtitude:String,
    pub uuid:String,
    pub date:String,
    pub typeg:String
}
