use crate::utils::deserialize_string_to_i32;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{Error, Result as SerdeResult, Value};

pub trait Table: DeserializeOwned {
    fn convert(value: Value) -> Result<Self, Error> {
        serde_json::from_value(value)
    }
}

// Implement Deserializable for any type that implements Deserialize
impl<T: DeserializeOwned> Table for T {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErgastResponse<T> {
    #[serde(rename = "MRData")]
    mrdata: MRData<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MRData<T> {
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
    table: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeasonTable {
    #[serde(rename = "Seasons")]
    seasons: Vec<Season>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RaceTable {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    season: i32,
    round: Option<String>,
    #[serde(rename = "Races")]
    races: Vec<Race>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QualifyingTable {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    season: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    round: i32,
    #[serde(rename = "Races")]
    races: Vec<Qualifying>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SprintTable {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    season: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    round: i32,
    #[serde(rename = "Races")]
    races: Vec<Sprint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandingTable {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    season: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    round: i32,
    #[serde(rename = "StandingsLists")]
    standings_lists: Vec<StandingsList>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriverTable {
    #[serde(rename = "Drivers")]
    drivers: Vec<Driver>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Season {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    season: i32,
    url: String,
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
    #[serde(rename = "Results")]
    results: Option<Vec<RaceResult>>,
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

// this is named "Result" in Ergast API. Renamed RaceResult not to mix up Result in Rust.
#[derive(Serialize, Deserialize, Debug)]
pub struct RaceResult {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    number: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    position: i32,
    #[serde(rename = "positionText")]
    position_text: String,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    points: i32,
    #[serde(rename = "Driver")]
    driver: Driver,
    #[serde(rename = "Constructor")]
    constructor: Constructor,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    grid: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    laps: i32,
    status: String,
    #[serde(rename = "Time")]
    time: Option<Time>,
    #[serde(rename = "FastestLap")]
    fastest_lap: FastestLap,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Driver {
    #[serde(rename = "driverId")]
    driver_id: String,
    #[serde(rename = "permanentNumber")]
    permanent_number: Option<String>,
    code: Option<String>,
    url: String,
    #[serde(rename = "givenName")]
    given_name: String,
    #[serde(rename = "familyName")]
    family_name: String,
    #[serde(rename = "dateOfBirth")]
    date_of_birth: String,
    nationality: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Constructor {
    #[serde(rename = "constructorId")]
    constructor_id: String,
    url: String,
    name: String,
    nationality: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    millis: Option<String>,
    time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FastestLap {
    rank: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    lap: i32,
    #[serde(rename = "Time")]
    time: Time,
    #[serde(rename = "AverageSpeed")]
    average_speed: Option<AverageSpeed>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AverageSpeed {
    units: String,
    speed: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Qualifying {
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
    #[serde(rename = "QualifyingResults")]
    qualifying_results: Vec<QualifyingResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QualifyingResult {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    number: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    position: i32,
    #[serde(rename = "Driver")]
    driver: Driver,
    #[serde(rename = "Constructor")]
    constructor: Constructor,
    #[serde(rename = "Q1")]
    q1: Option<String>,
    #[serde(rename = "Q2")]
    q2: Option<String>,
    #[serde(rename = "Q3")]
    q3: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sprint {
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
    #[serde(rename = "SprintResults")]
    sprint_results: Vec<RaceResult>, // use same data structure as race result
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandingsList {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    season: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    round: i32,
    #[serde(rename = "DriverStandings")]
    driver_standings: Vec<DriverStanding>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriverStanding {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    position: i32,
    #[serde(rename = "positionText")]
    position_text: String,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    points: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    wins: i32,
    #[serde(rename = "Driver")]
    driver: Driver,
    #[serde(rename = "Constructors")]
    constructors: Vec<Constructor>,
}

pub fn deserialize_mr_data<T: Table>(json: &str) -> SerdeResult<MRData<T>> {
    let value: Value = serde_json::from_str(json)?;
    let xmlns = value["MRData"]["xmlns"].as_str().unwrap().to_string();
    let series = value["MRData"]["series"].as_str().unwrap().to_string();
    let url = value["MRData"]["url"].as_str().unwrap().to_string();
    let limit: i32 = value["MRData"]["limit"]
        .as_str()
        .unwrap()
        .parse()
        .expect("Not a valid integer");
    let offset: i32 = value["MRData"]["offset"]
        .as_str()
        .unwrap()
        .parse()
        .expect("Not a valid integer");
    let total: i32 = value["MRData"]["total"]
        .as_str()
        .unwrap()
        .parse()
        .expect("Not a valid integer");

    let keys = ["xmlns", "series", "url", "limit", "offset", "total"];

    let key_finder = |val: Value| {
        if let Value::Object(map) = val {
            for (k, _v) in map {
                if !keys.contains(&k.as_str()) {
                    return Ok(k.to_owned());
                }
            }
        }
        Err("Cannot find key for data table".to_string())
    };
    let key_for_table = key_finder(value["MRData"].clone()).unwrap();

    Ok(MRData {
        xmlns,
        series,
        url,
        limit,
        offset,
        total,
        table: T::convert(value["MRData"][key_for_table].clone()).unwrap(),
    })
}
