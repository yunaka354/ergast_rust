use crate::api;
use crate::models::{
    deserialize_mr_data, MRData, QualifyingTable, RaceTable, SeasonTable, SprintTable, Table, StandingTable,
};
use reqwest::{Error, Response};

pub struct Ergast {}

impl Ergast {
    pub async fn seasons() -> Result<MRData<SeasonTable>, Error> {
        let response = api::API::get("seasons").await?;
        Ok(Ergast::handle_response::<SeasonTable>(response).await)
    }

    pub async fn race() -> Result<MRData<RaceTable>, Error> {
        let response = api::API::get("current").await?;
        Ok(Ergast::handle_response::<RaceTable>(response).await)
    }

    pub async fn results() -> Result<MRData<RaceTable>, Error> {
        let response = api::API::get("current/last/results").await?;
        Ok(Ergast::handle_response::<RaceTable>(response).await)
    }

    pub async fn qualifying(year: i32, round: i32) -> Result<MRData<QualifyingTable>, Error> {
        let url = format!("{year}/{round}/qualifying");
        let response = api::API::get(&url).await?;
        Ok(Ergast::handle_response::<QualifyingTable>(response).await)
    }

    pub async fn sprint(year: i32, round: i32) -> Result<MRData<SprintTable>, Error> {
        let url = format!("{year}/{round}/sprint");
        let response = api::API::get(&url).await?;
        Ok(Ergast::handle_response::<SprintTable>(response).await)
    }

    pub async fn standings(year: i32, round: i32) -> Result<MRData<StandingTable>, Error> {
        let url = format!("{year}/{round}/driverStandings");
        let response = api::API::get(&url).await?;
        Ok(Ergast::handle_response::<StandingTable>(response).await)
    }

    async fn handle_response<T: Table>(response: Response) -> MRData<T> {
        if response.status().is_success() {
            let json = response.text().await.unwrap();
            return deserialize_mr_data::<T>(&json).unwrap();
        } else {
            panic!("Failed to get data: {:?}", response.status());
        }
    }
}
