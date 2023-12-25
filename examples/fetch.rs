extern crate ergast_rust;

use ergast_rust::api;
use ergast_rust::ergast;

#[tokio::main]
async fn main() {
    let path = api::Path {
        year: 2023,
        round: Some(1),
    };
    let params = api::URLParams::default();
    let response = ergast::Ergast::results(path, params).await.unwrap();

    println!("{:?}", response);
}
