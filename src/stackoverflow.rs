use chrono::{Duration, Utc};
use reqwest::blocking::Client;
use serde_json::Value;

#[derive(Debug)]
pub struct StackOverflow {}

impl StackOverflow {
    pub fn new() -> StackOverflow {
        StackOverflow {}
    }
    pub fn search(&self, client: &Client, query: &str, days: u32) {
        let fromdate = Utc::now()
            .checked_sub_signed(Duration::days(days as i64))
            .expect("failed to shift date");

        let url = format!(
            "http://api.stackexchange.com/2.2/search/advanced?\
            order=desc&sort=creation&site=stackoverflow&title={query}&fromdate={timestamp}",
            query = query,
            timestamp = fromdate.timestamp(),
        );

        println!("Search topics since {} ...", fromdate);

        let res = match client.get(&url).send() {
            Ok(res) => res,
            Err(e) => {
                println!("Fetch url error :{} :{}", url, e);
                return;
            }
        };
        let data: Value = match res.json() {
            Ok(data) => data,
            Err(e) => {
                println!("Parse JSON error: {}", e);
                return;
            }
        };

        let items = match data.get("items") {
            Some(items) => items,
            None => {
                println!("Incorrect response, key `items` not found");
                return;
            }
        }
        .as_array()
        .unwrap();

        for item in items {
            println!("- {}", item["link"].as_str().unwrap());
        }

        if items.len() == 0 {
            println!("Nothin found.");
        }
    }
}
