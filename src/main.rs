#![allow(unused)]

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{borrow::Cow, error::Error};

trait INews: HostEndpointURLBuilder {
    fn fetch(&self) -> Result<GNewsStruct, Box<dyn Error>> {
        let full_url = self.build_url();
        // eprintln!("{}", full_url);
        let body = reqwest::blocking::get(full_url)?.text()?;
        let g: GNewsStruct = serde_json::from_str(&body)?;
        Ok(g)
    }
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GNewsStruct {
    total_articles: Value,
    articles: Vec<GNewsArticle>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GNewsArticle {
    title: String,
    description: String,
    url: String,
    image: String,
    published_at: String,
    source: GNewsArticleSource,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GNewsArticleSource {
    name: String,
    url: String,
}

pub trait HostEndpointURLBuilder {
    fn get_host_url(&self) -> String;
    fn get_host_endpoint(&self) -> String;
    fn get_host_endpoint_params(&self) -> String;
    fn build_url(&self) -> String {
        let params = if "" == self.get_host_endpoint_params() {
            "".to_owned() // Defaults to no params if the host / API allows it
        } else {
            let mut params = self.get_host_endpoint_params();
            let gnews_api_key = std::env::var("GNEWS_API_KEY").unwrap();
            params.push_str(&gnews_api_key);
            params
        };
        format!(
            "{}{}{}",
            self.get_host_url(),
            self.get_host_endpoint(),
            params,
        )
        .to_string()
    }
}

#[derive(Default)]
pub struct GNewsTopHeadlines {
    host_url: String,
    host_endpoint: String,
    host_endpoint_params: String,
}
const HOST_URL_DEFAULT: &str = "https://gnews.io";

const HOST_TOP_HEADLINES_ENDPOINT_DEFAULT: &str = "/api/v4/top-headlines";
const HOST_TOP_HEADLINES_ENDPOINT_PARAMS_DEFAULT: &str =
    "?category=general&lang=en&country=us&max=10&apikey=";
impl GNewsTopHeadlines {
    pub fn new() -> Self {
        Self {
            host_url: HOST_URL_DEFAULT.to_owned(),
            host_endpoint: HOST_TOP_HEADLINES_ENDPOINT_DEFAULT.to_string(),
            host_endpoint_params: HOST_TOP_HEADLINES_ENDPOINT_PARAMS_DEFAULT.into(),
        }
    }
}
impl HostEndpointURLBuilder for GNewsTopHeadlines {
    fn get_host_url(&self) -> String {
        self.host_url.to_string()
    }

    fn get_host_endpoint(&self) -> String {
        self.host_endpoint.to_string()
    }

    fn get_host_endpoint_params(&self) -> String {
        self.host_endpoint_params.to_owned()
    }
}
impl INews for GNewsTopHeadlines {}

pub struct GNewsSearch<'a> {
    host_url: Cow<'a, String>,
    host_endpoint: Cow<'a, String>,
    host_endpoint_params: Cow<'a, String>,
}
const HOST_SEARCH_ENDPOINT_DEFAULT: &str = "/api/v4/search";
impl GNewsSearch<'_> {
    pub fn new(
        search_term: &str,
        search_optional_fields_to_search: Option<String>,
        search_optional_countrycode: Option<String>,
        search_max_results: Option<u32>, // None or 1-10 for free gnews.io membership
    ) -> Self {
        let sth: String = "".to_owned();
        Self {
            host_url: Cow::Owned(HOST_URL_DEFAULT.to_owned()),
            host_endpoint: Cow::Owned(HOST_SEARCH_ENDPOINT_DEFAULT.to_owned()),
            host_endpoint_params: Cow::Owned(format!(
                "?q={}{}&lang=en&country={}&max={}&apikey=",
                search_term,
                search_optional_fields_to_search.map_or("".to_string(), |mut v| {
                    let mut r = String::from("&in=");
                    r.push_str(&v);
                    r
                }),
                search_optional_countrycode.unwrap_or("us".to_string()),
                search_max_results.unwrap_or(5)
            )),
        }
    }
    pub fn get_params(&mut self) -> String {
        self.host_endpoint_params.clone().into_owned()
    }
}
impl HostEndpointURLBuilder for GNewsSearch<'_> {
    fn get_host_url(&self) -> String {
        self.host_url.to_string()
    }

    fn get_host_endpoint(&self) -> String {
        self.host_endpoint.to_string()
    }

    fn get_host_endpoint_params(&self) -> String {
        self.host_endpoint_params.to_string()
    }
}
impl INews for GNewsSearch<'_> {}

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
