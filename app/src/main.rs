use reqwest::Client;
use scraper::{Html, Selector};
// use std::collections::HashMap;

const CUSTOM_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15";

#[tokio::main]
async fn main() {
    let ticker: &str = "AAPL";
    let url: &str = &format!("https://finance.yahoo.com/quote/{ticker}/analysis");

    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", CUSTOM_USER_AGENT)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let path_data: &str = "//*[@id='nimbus-app']/section/section/section/article/section[3]/div/table/tbody/tr[6]/td[4]";
    // let ticker_data: HashMap<&str, &str> = HashMap::new();
    let binding = Html::parse_document(&response);
    let rows = binding.select(&Selector::parse(path_data).unwrap())
        .map(| row| {
             row
                 .text()
        })
        .collect::<Vec<_>>();

    println!("{:?}", rows);
}
