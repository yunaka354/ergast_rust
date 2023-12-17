mod models;
mod utils;
mod api;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let response = api::API::get(api::EndPoint::Results).await?;

    if response.status().is_success() {
        let json = response.text().await?;
        let ergast_response = models::deserialize_mr_data::<models::RaceTable>(&json).unwrap();
        println!("{:?}", ergast_response);
    } else {
        println!("Failed to get data: {:?}", response.status());
    }

    Ok(())
}

