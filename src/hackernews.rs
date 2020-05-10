use chrono::{Duration, Utc};
use reqwest::blocking::Client;
use serde_json::Value;

#[derive(Debug)]
pub struct HackerNews {}

impl HackerNews {
    pub fn new() -> HackerNews {
        HackerNews {}
    }
    pub fn search(&self, client: &Client, query: &str, days: u32) {
        let fromdate = Utc::now()
            .checked_sub_signed(Duration::days(days as i64))
            .expect("failed to shift date");
        println!("Search topics since {} ...", fromdate);

        let url = format!(
            "http://hn.algolia.com/api/v1/search_by_date?tags=(story,comment)&\
            numericFilters=created_at_i>{timestamp}&query={query}",
            query = query,
            timestamp = fromdate.timestamp(),
        );

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

        let items = match data.get("hits") {
            Some(items) => items,
            None => {
                println!("Incorrect response, key `hits` not found");
                return;
            }
        }
        .as_array()
        .unwrap();

        for item in items {
            println!(
                "- https://news.ycombinator.com/item?id={}",
                item["objectID"].as_str().unwrap()
            )
        }

        if items.len() == 0 {
            println!("Nothin found.");
        }
    }
}
