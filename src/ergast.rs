use crate::api;
use crate::models::{
    deserialize_mr_data, MRData, QualifyingTable, RaceTable, SeasonTable, SprintTable,
    StandingTable, Table,
};
use reqwest::Error;

pub struct Ergast;

impl Ergast {
    pub async fn seasons() -> Result<MRData<SeasonTable>, Error> {
        Ok(Ergast::fetch::<SeasonTable>("seasons").await.unwrap())
    }

    pub async fn race() -> Result<MRData<RaceTable>, Error> {
        Ok(Ergast::fetch::<RaceTable>("current").await.unwrap())
    }

    pub async fn results() -> Result<MRData<RaceTable>, Error> {
        Ok(Ergast::fetch::<RaceTable>("current/last/results")
            .await
            .unwrap())
    }

    pub async fn qualifying(year: i32, round: i32) -> Result<MRData<QualifyingTable>, Error> {
        let url = format!("{year}/{round}/qualifying");
        Ok(Ergast::fetch::<QualifyingTable>(&url).await.unwrap())
    }

    pub async fn sprint(year: i32, round: i32) -> Result<MRData<SprintTable>, Error> {
        let url = format!("{year}/{round}/sprint");
        Ok(Ergast::fetch::<SprintTable>(&url).await.unwrap())
    }

    pub async fn standings(year: i32, round: i32) -> Result<MRData<StandingTable>, Error> {
        let url = format!("{year}/{round}/driverStandings");
        Ok(Ergast::fetch::<StandingTable>(&url).await.unwrap())
    }

    async fn fetch<T: Table>(url: &str) -> Result<MRData<T>, Error> {
        let response = api::API::get(&url).await?;
        if response.status().is_success() {
            let json = response.text().await.unwrap();
            return Ok(deserialize_mr_data::<T>(&json).unwrap());
        } else {
            panic!("Failed to get data: {:?}", response.status());
        }
    }
}
