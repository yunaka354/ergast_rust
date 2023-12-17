mod api;
mod ergast;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    let params = api::URLParams {
        limit: 10,
        offset: 0,
    };
    let data = ergast::Ergast::drivers(Some(params)).await.unwrap();
    println!("{:?}", data);
}
