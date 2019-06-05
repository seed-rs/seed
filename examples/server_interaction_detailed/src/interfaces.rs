use serde::{Deserialize, Serialize};

//use chrono::offset::{TimeZone, Utc};
//use chrono::Date;

// u32 generally refers to an id. i8 refers to a choice.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Line {
    pub id: u32,
    pub model: String,
    //    pub date: Date<Utc>
    pub date: String, // todo Date?
    ////    pilot: Person,
    pub pilot: Option<u32>,
    ////    wso: Person,
    pub wso: Option<u32>,
    pub number: u32,
    pub callsign: String,
    pub callsign_num: u32,
    pub mission2: Option<u32>,
    pub takeoff: String, // todo time?
    pub land: String,    // todo time?
    pub timeless: bool,
    pub flight_lead: bool,
    pub upgrade: bool,
    pub wso_upgrade: bool,
    pub pilot_ior: Option<u32>,
    pub wso_ior: Option<u32>,
    pub rap_counters: u32,
    pub notes: String,

    pub clone_of: Option<u32>,
    pub loadout: String,
    pub airspace: String,
    pub airspace_time: String,
    pub track_number: u32,
    pub fdl_time: String,
    pub tanker: String,

    pub upgrade_event: Option<u32>,
    pub ug_notes: Option<String>,
    pub ug_hide: bool,
    pub ug_gradesheet_complete: bool,

    pub mx_effective: i8,
    pub ug_effective: i8,
    pub guest_pilot: Option<String>,
    pub guest_wso: Option<String>,
    pub tdy: bool,
    pub cancelled: bool,

    pub loaded: Option<bool>, // Only used client-side
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Line2 {
    //    date: Date<Utc>,
    date: String,
    //    ..Line
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: u32,
    pub squadron: u32,
    pub last_name: String,
    pub first_name: String,
    pub flight: Option<String>,
    pub office_symbol: Option<String>,
    pub callsign: Option<String>,
    pub flying_callsign: Option<String>,
    pub job: i8,
    pub upgrade_level: i8,
    pub special_status: i8,
    pub rank: Option<i8>,
    pub active: bool,
    pub attached: bool,
    pub dnif: bool,
    pub lox_notes: Option<String>,
    pub crewmate: Option<u32>,
    pub api: i8,
    pub wx_cat: i8,
    pub lowat: i8,
    pub rap_requirement: (i8, i8),
    pub tdy: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Syllabus {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Upgrade {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeEvent {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Gradesheet {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mission {
    pub id: u32,
    pub name: String,
}
