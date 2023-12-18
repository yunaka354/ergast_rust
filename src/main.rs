mod api;
mod ergast;
mod models;
mod scripts;
mod utils;

#[tokio::main]
async fn main() {
    scripts::health_check::health_check().await;
}
