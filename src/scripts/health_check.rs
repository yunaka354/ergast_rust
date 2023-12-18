use crate::api::URLParams;
use crate::ergast::Ergast;

pub async fn health_check() -> () {

    // check seasons
    let seasons = Ergast::seasons(Some(URLParams::default())).await.unwrap();
    let seasons_total = seasons.total;
    let seasons_params = URLParams{limit: seasons_total, offset:0};
    let seasons = Ergast::seasons(Some(seasons_params)).await.unwrap();
    let seasons = seasons.table.seasons;

    // check race
    for season in seasons {
        let year = season.season;
        println!("fetch race() with year {year}");
        let params = URLParams {limit:100, offset:0};
        let response = Ergast::race(year, Some(params)).await.unwrap();
    }
}