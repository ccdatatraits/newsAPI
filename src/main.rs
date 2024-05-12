// #![allow(unused)]

// #[derive(Debug)]
// struct QEString(String);

// // impl Into<QEString<'_>> for &str {
// // fn into(self) -> QEString<'static> {
// //     QEString::<'_>(self)
// // }
// impl From<&str> for QEString {
//     fn from(value: &str) -> Self {
//         Self(String::from(value))
//     }
// }
// // impl AsRef<QEString> for &str {
// //     fn as_ref(&self) -> &QEString {
// //         &QEString(self.to_string())
// //     }
// // }

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::Value;
// use std::{borrow::Cow, error::Error, rc::Rc};
use std::error::Error;

// struct NewsIdentifier(usize);
// #[derive(Default)]
// struct NewsParameters(String);
// impl NewsParameters {
//     pub fn new() -> Self {
//         Self(
//             "?category=general&lang=en&country=us&max=10&apikey=<GO_TO_GNEWS_DASHBOARD_AND_COPY_PASTE_API_KEY_HERE>"
//                 .to_owned(),
//         )
//     }
// }

trait INews {
    // async fn fetch(identifier: NewsIdentifier);
    /*async */
    // fn fetch_top(&self) -> Result<String, Box<dyn Error>>;
    fn fetch_top(&self) -> Result<GNewsStruct, Box<dyn Error>>;
}

impl INews for GNews {
    /*async */
    // fn fetch_top(&self) -> Result<String, Box<dyn Error>> {
    fn fetch_top(&self) -> Result<GNewsStruct, Box<dyn Error>> {
        let full_url = self.build_url();
        let body = reqwest::blocking::get(full_url)?.text()?;
        // let v: Value = serde_json::from_str(&body)?;
        let g: GNewsStruct = serde_json::from_str(&body)?;
        // Ok(GNewsStruct {
        //     total_articles: v["totalArticles"].clone(),
        //     articles: v["articles"].clone(),
        // })
        Ok(g)
    }
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GNewsStruct {
    total_articles: Value,
    articles: Vec<GNewsArticle>,
}
// pub struct GNewsStruct(Value);

// fn convert_json_to_struct() {
//     // create a raw JSON string from the json! macro and turn it into a MyStruct struct
//     let raw_json_string = json!({"message": "Hello world!"});
//     let my_struct: MyStruct = serde_json::from_str(raw_json_string).unwrap();
// }
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
// impl GNewsArticle {
//     pub fn new() -> Self {
//         Self {
//             ..Default::default()
//         }
//     }
// }

#[derive(Default)]
pub struct GNews {
    // host_url: Cow<'static, str>,
    // host_endpoint: Cow<'static, str>,
    // host_endpoint_params: Cow<'static, str>,
    host_url: String,
    host_endpoint: String,
    host_endpoint_params: String,
}
const HOST_URL_DEFAULT: &str = "https://gnews.io";
const HOST_ENDPOINT_DEFAULT: &str = "/api/v4/top-headlines";
const HOST_ENDPOINT_PARAMS_DEFAULT: &str = "?category=general&lang=en&country=us&max=10&apikey=";
impl GNews {
    // const HOST_URL_DEFAULT: Cow<'static, str> = Cow::Borrowed("https://gnews.io");
    // const HOST_ENDPOINT_DEFAULT: Cow<'static, str> = Cow::Borrowed("/api/v4/top-headlines");
    // const HOST_ENDPOINT_PARAMS_DEFAULT: Cow<'static, str> =
    //     Cow::Borrowed(format!("?category=general&lang=en&country=us&max=10&apikey=").as_ref());
    pub fn new() -> Self {
        Self {
            host_url: HOST_URL_DEFAULT.to_owned(),
            host_endpoint: HOST_ENDPOINT_DEFAULT.to_string(),
            host_endpoint_params: HOST_ENDPOINT_PARAMS_DEFAULT.into(),
            ..Default::default()
        }
    }
    pub fn build_url(&self) -> String {
        let mut params = self.host_endpoint_params.clone();
        let gnews_api_key = std::env::var("GNEWS_API_KEY").unwrap();
        params.push_str(&gnews_api_key);
        format!(
            "{}{}{}",
            self.host_url.clone(),
            self.host_endpoint.clone(),
            params,
        )
        .to_string()
    }
}

fn main() {
    dotenv().ok();

    println!("NEWS Aggregator");

    // let n = GNews::new().build_url();
    // println!("{:?}", n);
    let gnews = GNews::new();
    let gnews_top_news = gnews.fetch_top();
    println!("{gnews_top_news:?}");
}
