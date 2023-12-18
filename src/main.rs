mod api;
mod ergast;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    let params = api::URLParams {
        limit: 1,
        offset: 0,
    };
    let data = ergast::Ergast::status(Some(params)).await.unwrap();

    let pretty_json = serde_json::to_string_pretty(&data)
        .expect("Failed to serialize object");

    println!("{}", pretty_json);
}
