mod models;
mod utils;
mod api;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let response = api::API::get(api::EndPoint::Race).await?;

    if response.status().is_success() {
        let ergast_response: models::ErgastResponse = response.json().await?;
        println!("{:?}", ergast_response);
    } else {
        println!("Failed to get data: {:?}", response.status());
    }

    Ok(())
}
