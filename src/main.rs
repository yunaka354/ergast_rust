mod api;
mod ergast;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    let _ = ergast::Ergast::seasons().await.unwrap();
    let _ = ergast::Ergast::race().await.unwrap();
    let _ = ergast::Ergast::results().await.unwrap();
    let data = ergast::Ergast::qualifying(2023, 5).await.unwrap();
    println!("{:?}", data);
}
