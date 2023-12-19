use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;

use reqwest::Error;

use crate::api::{self, Path, URLParams};
use crate::ergast::Ergast;
use crate::models::MRData;

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

    // set enough limit. (I don't think F1 has more than 100 rounds in a year...)
    // let params = URLParams {
    //     limit: 1000,
    //     offset: 0,
    // };
    // for (year, races) in map.into_iter() {
    //     for race in races {
    //         let round = race.round;
    //         println!("fetch result() for year {year}, round {round}");
    //         let _response = Ergast::results(year, round, Some(params)).await.unwrap();
    //     }
    // }

    // check qualifying
    // iterate_over(Ergast::qualifying).await;

    // check sprint
    // iterate_over(Ergast::sprint).await;

    // check standings
    // very heavy. do not use.
    // iterate_over(Ergast::standings).await;

    // check drivers
    // let params = URLParams{limit:1000, offset:0};
    // Ergast::drivers(Some(params)).await.unwrap();

    // check constructors
    // let params = URLParams{limit:1000, offset:0};
    // Ergast::constructors(Some(params)).await.unwrap();

    // check circuits
    // let params = URLParams{limit:1000, offset:0};
    // Ergast::circuits(Some(params)).await.unwrap();

    // check status
    // let params = URLParams{limit:1000, offset:0};
    // Ergast::status(Some(params)).await.unwrap();

    // check laps (* this does not do paging. meaning, not to check all data.)
    // let params = URLParams {
    //     limit: 1000,
    //     offset: 0,
    // };
    // for (year, races) in map.into_iter() {
    //     if year < 1996 {
    //         // laps data is available 1996 onwards.
    //         continue;
    //     }
    //     for race in races {
    //         let round = race.round;
    //         let path = Path {
    //             year,
    //             round: Some(round),
    //         };
    //         println!("fetch laps() for year {year}, round {round}");
    //         let _response = Ergast::laps(Some(path), Some(params)).await.unwrap();
    //     }
    // }

    // check pitstops (* this does not do paging. meaning, not to check all data.)
    let params = URLParams {
        limit: 1000,
        offset: 0,
    };
    for (year, races) in map.into_iter() {
        if year < 2012 {
            // pitstop data is available 1996 onwards.
            continue;
        }
        for race in races {
            let round = race.round;
            let path = Path {
                year,
                round: Some(round),
            };
            println!("fetch pitstops() for year {year}, round {round}");
            let _response = Ergast::pitstops(Some(path), Some(params)).await.unwrap();
        }
    }
}

async fn iterate_over<F, Fut, T>(f: F) -> ()
where
    F: Fn(Option<api::Path>, Option<api::URLParams>) -> Fut,
    Fut: Future<Output = Result<MRData<T>, Error>>,
    T: Debug,
{
    let limit = 1000;
    let response = f(None, None).await.unwrap();
    let total = response.total;
    let pages = (total + limit - 1) / limit; // round up

    for i in 0..pages {
        let offset = limit * i;
        let params = URLParams { limit, offset };
        let response = f(None, Some(params)).await.unwrap();
        println!("{:?}", params);
        println!("{:?}", response);
    }
}
