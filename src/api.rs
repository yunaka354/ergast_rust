use reqwest::{Response, Error};

#[allow(dead_code)]
pub enum EndPoint {
    Seasons,
    Race,
    Results,
}

impl EndPoint {
    fn to_string(&self) -> &str {
        match self {
            EndPoint::Seasons => "seasons",
            EndPoint::Race => "current",
            EndPoint::Results => "current/last/results",
        }
    }
}

pub struct API {}

impl API {
    pub async fn get(endpoint: EndPoint) -> Result<Response, Error> {
        let url = format!("https://ergast.com/api/f1/{}.json", endpoint.to_string());
        reqwest::get(url).await
    }
}