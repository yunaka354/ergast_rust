use crate::api;
use crate::models::{
    deserialize_mr_data, DriverTable, MRData, QualifyingTable, RaceTable, SeasonTable, SprintTable,
    StandingTable, Table, ConstructorTable,
};
use reqwest::Error;

pub struct Ergast;

#[allow(dead_code)]
impl Ergast {
    pub async fn seasons() -> Result<MRData<SeasonTable>, Error> {
        Ok(Ergast::fetch::<SeasonTable>("seasons", None).await.unwrap())
    }

    pub async fn race() -> Result<MRData<RaceTable>, Error> {
        Ok(Ergast::fetch::<RaceTable>("current", None).await.unwrap())
    }

    pub async fn results() -> Result<MRData<RaceTable>, Error> {
        Ok(Ergast::fetch::<RaceTable>("current/last/results", None)
            .await
            .unwrap())
    }

    pub async fn qualifying(year: i32, round: i32) -> Result<MRData<QualifyingTable>, Error> {
        let url = format!("{year}/{round}/qualifying");
        Ok(Ergast::fetch::<QualifyingTable>(&url, None).await.unwrap())
    }

    pub async fn sprint(year: i32, round: i32) -> Result<MRData<SprintTable>, Error> {
        let url = format!("{year}/{round}/sprint");
        Ok(Ergast::fetch::<SprintTable>(&url, None).await.unwrap())
    }

    pub async fn standings(year: i32, round: i32) -> Result<MRData<StandingTable>, Error> {
        let url = format!("{year}/{round}/driverStandings");
        Ok(Ergast::fetch::<StandingTable>(&url, None).await.unwrap())
    }

    pub async fn drivers(params: Option<api::URLParams>) -> Result<MRData<DriverTable>, Error> {
        Ok(Ergast::fetch::<DriverTable>("drivers", params).await.unwrap())
    }

    pub async fn constructors(params: Option<api::URLParams>) -> Result<MRData<ConstructorTable>, Error> {
        Ok(Ergast::fetch::<ConstructorTable>("constructors", params).await.unwrap())
    }

    async fn fetch<T: Table>(
        url: &str,
        params: Option<api::URLParams>,
    ) -> Result<MRData<T>, Error> {
        let response = api::API::get(&url, params).await?;
        if response.status().is_success() {
            let json = response.text().await.unwrap();
            return Ok(deserialize_mr_data::<T>(&json).unwrap());
        } else {
            panic!("Failed to get data: {:?}", response.status());
        }
    }
}
