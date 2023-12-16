use serde::{Deserialize, Serialize};
use serde_json::{Result as SerdeResult, Value, from_value};
use crate::utils::deserialize_string_to_i32;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")] // Use a tag to distinguish between types
pub enum Table {
    SeasonTable(SeasonTable),
    RaceTable(RaceTable)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErgastResponse<T> {
    #[serde(rename = "MRData")]
    mrdata: MRData<T>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MRData<T>
{
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
    table: T
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

pub fn deserialize_mr_data(json: &str) -> SerdeResult<MRData<Table>> {
    let value: Value = serde_json::from_str(json)?;
    
    let xmlns = value["MRData"]["xmlns"].as_str().unwrap().to_string();
    let series = value["MRData"]["series"].as_str().unwrap().to_string();
    let url = value["MRData"]["url"].as_str().unwrap().to_string();
    let limit: i32 = value["MRData"]["limit"].as_str().unwrap().parse().expect("Not a valid integer");
    let offset: i32 = value["MRData"]["offset"].as_str().unwrap().parse().expect("Not a valid integer");
    let total: i32 = value["MRData"]["total"].as_str().unwrap().parse().expect("Not a valid integer");

    let mut table = None;

    let keys = [
        "xmlns",
        "series",
        "url",
        "limit",
        "offset",
        "total",
    ];

    if let Value::Object(map) = &value["MRData"] {
        for (k, v) in map {
            if !keys.contains(&k.as_str()) {
                println!("key is {k}");
                table = if let Ok(table) = from_value::<SeasonTable>(v.clone()) {
                    Some(Table::SeasonTable(table))
                } else if let Ok(table) = from_value::<RaceTable>(v.clone()) {
                    Some(Table::RaceTable(table))
                } else {
                    None
                };
                break;
            }
        }
    }

    Ok(MRData {
        xmlns,
        series,
        url,
        limit,
        offset,
        total,
        table: table.expect("Failed to deserialize dynamic table"),
    })
}