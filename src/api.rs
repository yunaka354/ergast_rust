use reqwest::{Error, Response};

#[derive(Clone, Copy, Debug)]
pub struct URLParams {
    pub limit: i32,
    pub offset: i32,
}

impl Default for URLParams {
    fn default() -> Self {
        URLParams {
            limit: 30, // default setting for Ergast server
            offset: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Path {
    pub year: i32,
    pub round: Option<i32>,
}

pub struct API {}

impl API {
    pub async fn get(url: &str, params: Option<URLParams>) -> Result<Response, Error> {
        let mut url = format!("https://ergast.com/api/f1/{}.json", url);
        // if params is thrown, add parameters to url.
        if let Some(p) = params {
            let params = format!("?limit={}&offset={}", p.limit, p.offset);
            url.push_str(&params);
        }
        println!("get...{url}");
        reqwest::get(url).await
    }
}
