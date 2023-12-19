use crate::api;
use crate::models::{
    deserialize_mr_data, CircuitTable, ConstructorTable, DriverTable, MRData, QualifyingTable,
    RaceTable, SeasonTable, SprintTable, StandingTable, StatusTable, Table,
};
use reqwest::Error;

pub struct Ergast;

#[allow(dead_code)]
impl Ergast {
    pub async fn seasons(params: Option<api::URLParams>) -> Result<MRData<SeasonTable>, Error> {
        Ok(Ergast::fetch::<SeasonTable>("seasons", params)
            .await
            .unwrap())
    }

    pub async fn race(
        year: i32,
        params: Option<api::URLParams>,
    ) -> Result<MRData<RaceTable>, Error> {
        let url = format!("{year}");
        Ok(Ergast::fetch::<RaceTable>(&url, params).await.unwrap())
    }

    pub async fn results(
        path: Option<api::Path>,
        params: Option<api::URLParams>,
    ) -> Result<MRData<RaceTable>, Error> {
        let url = Ergast::build_path("results", path);
        Ok(Ergast::fetch::<RaceTable>(&url, params).await.unwrap())
    }

    pub async fn qualifying(
        path: Option<api::Path>,
        params: Option<api::URLParams>,
    ) -> Result<MRData<QualifyingTable>, Error> {
        let url = Ergast::build_path("qualifying", path);
        Ok(Ergast::fetch::<QualifyingTable>(&url, params)
            .await
            .unwrap())
    }

    pub async fn sprint(
        path: Option<api::Path>,
        params: Option<api::URLParams>,
    ) -> Result<MRData<SprintTable>, Error> {
        let url = Ergast::build_path("sprint", path);
        Ok(Ergast::fetch::<SprintTable>(&url, params).await.unwrap())
    }

    pub async fn standings(
        path: Option<api::Path>,
        params: Option<api::URLParams>,
    ) -> Result<MRData<StandingTable>, Error> {
        let url = Ergast::build_path("driverStandings", path);
        Ok(Ergast::fetch::<StandingTable>(&url, params).await.unwrap())
    }

    pub async fn drivers(params: Option<api::URLParams>) -> Result<MRData<DriverTable>, Error> {
        Ok(Ergast::fetch::<DriverTable>("drivers", params)
            .await
            .unwrap())
    }

    pub async fn constructors(
        params: Option<api::URLParams>,
    ) -> Result<MRData<ConstructorTable>, Error> {
        Ok(Ergast::fetch::<ConstructorTable>("constructors", params)
            .await
            .unwrap())
    }

    pub async fn circuits(params: Option<api::URLParams>) -> Result<MRData<CircuitTable>, Error> {
        Ok(Ergast::fetch::<CircuitTable>("circuits", params)
            .await
            .unwrap())
    }

    pub async fn status(params: Option<api::URLParams>) -> Result<MRData<StatusTable>, Error> {
        Ok(Ergast::fetch::<StatusTable>("status", params)
            .await
            .unwrap())
    }

    pub async fn laps(
        path: Option<api::Path>,
        params: Option<api::URLParams>,
    ) -> Result<MRData<RaceTable>, Error> {
        let url = Ergast::build_path("laps", path);
        Ok(Ergast::fetch::<RaceTable>(&url, params).await.unwrap())
    }

    pub async fn pitstops(
        path: Option<api::Path>,
        params: Option<api::URLParams>,
    ) -> Result<MRData<RaceTable>, Error> {
        let url = Ergast::build_path("pitstops", path);
        Ok(Ergast::fetch::<RaceTable>(&url, params).await.unwrap())
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

    fn build_path(endpoint: &str, path: Option<api::Path>) -> String {
        match path {
            Some(p) => {
                let year = p.year.to_string();
                match p.round {
                    // if year and round are specified, concatinate all.
                    Some(r) => {
                        let round = r.to_string();
                        format!("{year}/{round}/{endpoint}")
                    }
                    // if only year is specified, specify year only.
                    None => format!("{year}/{endpoint}"),
                }
            }
            // if path is not specified, use endpoint only.
            None => {
                format!("{endpoint}")
            }
        }
    }
}
