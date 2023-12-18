mod api;
mod ergast;
mod models;
mod utils;
mod scripts;

#[tokio::main]
async fn main() {
    scripts::health_check::health_check().await;
}
