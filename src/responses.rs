use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct TrainResponse(pub HashMap<String, Vec<Train>>);

#[derive(Debug, Deserialize, Clone)]
pub struct Train {
    #[serde(rename = "routeName")]
    pub route_name: String,

    #[serde(rename = "trainNum")]
    pub train_num: u32,

    #[serde(rename = "trainID")]
    pub train_id: String,

    pub lat: f64,

    pub lon: f64,

    #[serde(rename = "trainTimely")]
    pub train_timely: String,

    pub stations: Vec<TrainStation>,

    pub heading: String,

    #[serde(rename = "eventCode")]
    pub event_code: String,

    #[serde(rename = "eventTZ")]
    pub event_tz: String,

    #[serde(rename = "eventName")]
    pub event_name: String,

    #[serde(rename = "origCode")]
    pub origin_code: String,

    #[serde(rename = "originTZ")]
    pub origin_tz: String,

    #[serde(rename = "origName")]
    pub origin_name: String,

    #[serde(rename = "destCode")]
    pub destination_code: String,

    #[serde(rename = "destTZ")]
    pub destination_tz: String,

    #[serde(rename = "destName")]
    pub destination_name: String,

    #[serde(rename = "trainState")]
    pub train_state: TrainState,

    pub velocity: f32,

    #[serde(rename = "statusMsg")]
    pub status_message: String,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<FixedOffset>,

    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<FixedOffset>,

    #[serde(rename = "lastValTS")]
    pub last_value: DateTime<FixedOffset>,

    #[serde(rename = "objectID")]
    pub object_id: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TrainStation {
    pub name: String,
    pub code: String,
    pub tz: String,
    pub bus: bool,

    #[serde(rename = "schArr")]
    pub schedule_arrival: DateTime<FixedOffset>,

    #[serde(rename = "schDep")]
    pub schedule_departure: DateTime<FixedOffset>,

    #[serde(rename = "arr", default)]
    pub arrival: Option<DateTime<FixedOffset>>,

    #[serde(rename = "dep", default)]
    pub departure: Option<DateTime<FixedOffset>>,

    #[serde(rename = "arrCmnt")]
    pub arrival_comment: String,

    #[serde(rename = "depCmnt")]
    pub departure_comment: String,

    pub status: TrainStatus,
}

#[derive(Debug, Deserialize, Copy, Clone, PartialEq, Eq)]
pub enum TrainStatus {
    Enroute,
    Station,
    Departed,
    Unknown,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub enum TrainState {
    Predeparture,
    Active,
    Complete,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StationResponse(pub HashMap<String, Station>);

#[derive(Debug, Deserialize, Clone)]
pub struct Station {
    pub name: String,
    pub code: String,
    pub tz: String,
    pub lat: f64,
    pub lon: f64,
    pub address1: String,
    pub address2: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub trains: Vec<String>,
}
