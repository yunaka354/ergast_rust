use serde::{Deserialize, Serialize};
use crate::utils::deserialize_string_to_i32;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")] // Use a tag to distinguish between types
pub enum Table {
    SeasonTable(SeasonTable),
    RaceTable(RaceTable)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErgastResponse {
    #[serde(rename = "MRData")]
    mrdata: MRData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MRData {
    xmlns: String,
    series: String,
    url: String,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    limit: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    offset: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    total: i32,
    #[serde(rename = "RaceTable")]
    race_table: RaceTable
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeasonTable {
    #[serde(rename = "Seasons")]
    seasons: Vec<Season>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Season {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    season: i32,
    url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RaceTable {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    season: i32,
    #[serde(rename = "Races")]
    races: Vec<Race>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Race {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    season: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    round: i32,
    url: String,
    #[serde(rename = "raceName")]
    race_name: String,
    #[serde(rename = "Circuit")]
    circuit: Circuit,
    date: String,
    time: String,
    #[serde(rename = "FirstPractice")]
    first_practice: Option<Schedule>,
    #[serde(rename = "SecondPractice")]
    second_practice: Option<Schedule>,
    #[serde(rename = "ThirdPractice")]
    third_practice: Option<Schedule>,
    #[serde(rename = "Qualifying")]
    qualifying: Option<Schedule>,
    #[serde(rename = "Sprint")]
    sprint: Option<Schedule>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Circuit {
    #[serde(rename = "circuitId")]
    circuit_id: String,
    url: String,
    #[serde(rename = "circuitName")]
    circuit_name: String,
    #[serde(rename = "Location")]
    location: Location,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    lat: String,
    long: String,
    locality: String,
    country: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Schedule {
    date: String,
    time: String,
}
