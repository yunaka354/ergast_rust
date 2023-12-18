use std::collections::HashMap;

use crate::api::URLParams;
use crate::ergast::Ergast;

pub async fn health_check() -> () {
    // check seasons
    let seasons = Ergast::seasons(Some(URLParams::default())).await.unwrap();
    let seasons_total = seasons.total;
    let seasons_params = URLParams {
        limit: seasons_total,
        offset: 0,
    };
    let seasons = Ergast::seasons(Some(seasons_params)).await.unwrap();
    let seasons = seasons.table.seasons;

    // check race
    let mut map = HashMap::new();
    for season in seasons {
        let year = season.season;
        println!("fetch race() with year {year}");
        let params = URLParams {
            limit: 100,
            offset: 0,
        };
        let response = Ergast::race(year, Some(params)).await.unwrap();
        map.insert(year, response.table.races); // store data for next checking
    }

    // check results
    let params = URLParams {
        limit: 100,
        offset: 0,
    };
    
    // set enough limit. (I don't think F1 has more than 100 rounds in a year...)
    for (year, races) in map.into_iter() {
        for race in races {
            let round = race.round;
            println!("fetch result() for year {year}, round {round}");
            let _response = Ergast::results(year, round, Some(params)).await.unwrap();
        }
    }
}
