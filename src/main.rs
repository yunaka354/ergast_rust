
#[tokio::main]
async fn main() {
    let body = reqwest::get("https://ergast.com/api/f1/seasons.json")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{:?}", body);
}
