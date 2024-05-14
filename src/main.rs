#![allow(unused)]

use dotenv::dotenv;
use news_api::qemod::*;

fn main() {
    dotenv().ok();

    println!("\nNEWS Aggregator\n");
    let mut gnews = GNewsSearch::new(
        "quantum",
        Some("title,description".to_owned()),
        Some("au".to_string()),
        Some(5),
    );
    // println!("{}", gnews.get_params());
    let gnews_news = gnews.fetch();
    println!("{gnews_news:#?}");
}
