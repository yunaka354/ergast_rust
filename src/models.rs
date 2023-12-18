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
    pub mrdata: MRData<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MRData<T> {
    pub xmlns: String,
    pub series: String,
    pub url: String,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub limit: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub offset: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub total: i32,
    #[serde(rename = "RaceTable")]
    pub table: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeasonTable {
    #[serde(rename = "Seasons")]
    pub seasons: Vec<Season>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RaceTable {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub season: i32,
    pub round: Option<String>,
    #[serde(rename = "Races")]
    pub races: Vec<Race>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QualifyingTable {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub season: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub round: i32,
    #[serde(rename = "Races")]
    pub races: Vec<Qualifying>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SprintTable {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub season: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub round: i32,
    #[serde(rename = "Races")]
    pub races: Vec<Sprint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandingTable {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub season: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub round: i32,
    #[serde(rename = "StandingsLists")]
    pub standings_lists: Vec<StandingsList>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriverTable {
    #[serde(rename = "Drivers")]
    pub drivers: Vec<Driver>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConstructorTable {
    #[serde(rename = "Constructors")]
    pub constructors: Vec<Constructor>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CircuitTable {
    #[serde(rename = "Circuits")]
    pub circuits: Vec<Circuit>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusTable {
    #[serde(rename = "Status")]
    pub status: Vec<Status>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Season {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub season: i32,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Race {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub season: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub round: i32,
    pub url: String,
    #[serde(rename = "raceName")]
    pub race_name: String,
    #[serde(rename = "Circuit")]
    pub circuit: Circuit,
    pub date: String,
    pub time: Option<String>,
    #[serde(rename = "Results")]
    pub results: Option<Vec<RaceResult>>,
    #[serde(rename = "FirstPractice")]
    pub first_practice: Option<Schedule>,
    #[serde(rename = "SecondPractice")]
    pub second_practice: Option<Schedule>,
    #[serde(rename = "ThirdPractice")]
    pub third_practice: Option<Schedule>,
    #[serde(rename = "Qualifying")]
    pub qualifying: Option<Schedule>,
    #[serde(rename = "Sprint")]
    pub sprint: Option<Schedule>,
    #[serde(rename = "Laps")]
    pub laps: Option<Vec<Lap>>,
    #[serde(rename = "PitStops")]
    pub pitstops: Option<Vec<Pitstop>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Circuit {
    #[serde(rename = "circuitId")]
    pub circuit_id: String,
    pub url: String,
    #[serde(rename = "circuitName")]
    pub circuit_name: String,
    #[serde(rename = "Location")]
    pub location: Location,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub lat: String,
    pub long: String,
    pub locality: String,
    pub country: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Schedule {
    pub date: String,
    pub time: Option<String>,
}

// this is named "Result" in Ergast API. Renamed RaceResult not to mix up Result in Rust.
#[derive(Serialize, Deserialize, Debug)]
pub struct RaceResult {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub number: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub position: i32,
    #[serde(rename = "positionText")]
    pub position_text: String,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub points: i32,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub grid: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub laps: i32,
    pub status: String,
    #[serde(rename = "Time")]
    pub time: Option<Time>,
    #[serde(rename = "FastestLap")]
    pub fastest_lap: FastestLap,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Driver {
    #[serde(rename = "driverId")]
    pub driver_id: String,
    #[serde(rename = "permanentNumber")]
    pub permanent_number: Option<String>,
    pub code: Option<String>,
    pub url: String,
    #[serde(rename = "givenName")]
    pub given_name: String,
    #[serde(rename = "familyName")]
    pub family_name: String,
    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: String,
    pub nationality: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Constructor {
    #[serde(rename = "constructorId")]
    pub constructor_id: String,
    pub url: String,
    pub name: String,
    pub nationality: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    pub millis: Option<String>,
    pub time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FastestLap {
    pub rank: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub lap: i32,
    #[serde(rename = "Time")]
    pub time: Time,
    #[serde(rename = "AverageSpeed")]
    pub average_speed: Option<AverageSpeed>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AverageSpeed {
    pub units: String,
    pub speed: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Qualifying {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub season: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub round: i32,
    pub url: String,
    #[serde(rename = "raceName")]
    pub race_name: String,
    #[serde(rename = "Circuit")]
    pub circuit: Circuit,
    pub date: String,
    pub time: String,
    #[serde(rename = "QualifyingResults")]
    pub qualifying_results: Vec<QualifyingResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QualifyingResult {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub number: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub position: i32,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
    #[serde(rename = "Q1")]
    pub q1: Option<String>,
    #[serde(rename = "Q2")]
    pub q2: Option<String>,
    #[serde(rename = "Q3")]
    pub q3: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sprint {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub season: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub round: i32,
    pub url: String,
    #[serde(rename = "raceName")]
    pub race_name: String,
    #[serde(rename = "Circuit")]
    pub circuit: Circuit,
    pub date: String,
    pub time: String,
    #[serde(rename = "SprintResults")]
    pub sprint_results: Vec<RaceResult>, // use same data structure as race result
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StandingsList {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub season: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub round: i32,
    #[serde(rename = "DriverStandings")]
    pub driver_standings: Vec<DriverStanding>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriverStanding {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub position: i32,
    #[serde(rename = "positionText")]
    pub position_text: String,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub points: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub wins: i32,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructors")]
    pub constructors: Vec<Constructor>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    #[serde(rename = "statusId", deserialize_with = "deserialize_string_to_i32")]
    pub status_id: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub count: i32,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lap {
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub number: i32,
    #[serde(rename = "Timings")]
    pub timings: Vec<Timing>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timing {
    #[serde(rename = "driverId")]
    pub driver_id: String,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub position: i32,
    pub time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pitstop {
    #[serde(rename = "driverId")]
    pub driver_id: String,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub lap: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub stop: i32,
    pub time: String,
    pub duration: String,
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
