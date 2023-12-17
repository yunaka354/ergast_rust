use reqwest::{Error, Response};

pub struct API {}

impl API {
    pub async fn get(url: &str) -> Result<Response, Error> {
        let url = format!("https://ergast.com/api/f1/{}.json", url);
        reqwest::get(url).await
    }
}
