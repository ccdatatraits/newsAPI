#![allow(unused)]

use axum::{routing::get, Router};
use dotenv::dotenv;
use news_api::qemod::*;

// mod news_api_tests {
//     use news_api::qemod::*;
//     use std::error::Error;

//     #[test]
//     fn check_five_articles() -> Result<(), Box<dyn Error>> {
//         let mut gnews = GNewsTopHeadlines::new();
//         let res = gnews.fetch().unwrap_or_default();
//         // println!("{:#?}", gnews);
//         // let gnews = gnews.get_host_url();
//         // debug_assert!(true, "{:?}", gnews.get_host_endpoint());
//         debug_assert_eq!(res.get_total_articles(), 95493);
//         Ok(())
//     }
// }
async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    dotenv().ok();

    println!("\nNEWS Aggregator\n");

    // let gnews_top = GNewsTopHeadlines::new().fetch();
    // println!("{:#?}", gnews_top);

    // let mut gnews = GNewsSearch::new(
    //     "quantum",
    //     Some("title,description".to_owned()),
    //     Some("au".to_string()),
    //     Some(5),
    // );
    // // println!("{}", gnews.get_params());
    // let gnews_search = gnews.fetch();
    // println!("{:#?}", gnews_search);
    let router = Router::new().route(
        "/", get(hello_world)
    );

    Ok(router.into())
}
