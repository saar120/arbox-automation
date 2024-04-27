use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LessonsOpenType {
    TwentyFourHours,
    FixedWeekly,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoxInfo {
    id: u32,
    tz: String,
    location: String,
    pub(crate) lessons_open_type: LessonsOpenType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schedule {
    pub(crate) day: String,
    pub(crate) time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) schedule: Vec<Schedule>,
    pub(crate) box_info: BoxInfo,
}
