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
    let data = ergast::Ergast::standings(2020, 1, Some(params))
        .await
        .unwrap();

    println!("{:?}", data);
}
