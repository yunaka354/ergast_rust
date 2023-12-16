use serde::{Deserialize, Serialize};
use crate::utils::deserialize_string_to_i32;

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
    #[serde(rename = "SeasonTable")]
    season_table: Seasons
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Seasons {
    #[serde(rename = "Seasons")]
    seasons: Vec<Season>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Season {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    season: i32,
    url: String
}