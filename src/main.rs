mod api;
mod ergast;
mod models;
mod scripts;
mod utils;

#[tokio::main]
async fn main() {
    let path = api::Path{year:2023, round:Some(1)};
    let params = api::URLParams::default();
    ergast::Ergast::results(path, params).await.unwrap();
}
