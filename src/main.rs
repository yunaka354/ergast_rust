mod api;
mod ergast;
mod models;
mod scripts;
mod utils;

#[tokio::main]
async fn main() {
    let path = api::Path{year:2008, round:Some(5)};
    let params = api::URLParams::default();
    ergast::Ergast::results(Some(path), Some(params)).await.unwrap();
}
