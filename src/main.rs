mod models;
mod utils;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let response = reqwest::get("https://ergast.com/api/f1/seasons.json").await?;

    if response.status().is_success() {
        let ergast_response: models::ErgastResponse = response.json().await?;
        println!("{:?}", ergast_response);
    } else {
        println!("Failed to get data: {:?}", response.status());
    }

    Ok(())
}
