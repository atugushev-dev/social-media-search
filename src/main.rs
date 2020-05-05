extern crate chrono;
extern crate reqwest;

use chrono::{Duration, Utc};
use serde_json::Value;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "social-media-search")]
enum Command {
    /// Stack Overflow
    #[structopt(name = "so")]
    StackOverflow {
        query: String,

        #[structopt(short, long, required = false, default_value = "1")]
        days: i64,
    },
    /// Twitter
    #[structopt(name = "tw")]
    Twitter { query: String },
    /// Hacker News
    #[structopt(name = "hn")]
    HackerNews { query: String },
}

fn main() {
    let client = reqwest::blocking::Client::builder()
        .gzip(true)
        .build()
        .unwrap();

    match Command::from_args() {
        Command::StackOverflow { query, days } => {
            let fromdate = Utc::now()
                .checked_sub_signed(Duration::days(days))
                .unwrap()
                .timestamp();

            let url = format!(
                "http://api.stackexchange.com/2.2/search/advanced?\
                order=desc&sort=creation&site=stackoverflow&title={}&fromdate={}",
                query, fromdate
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

            for item in data["items"].as_array().unwrap() {
                println!("- {}", item["link"].as_str().unwrap());
            }
        }
        Command::Twitter { query } => {
            println!("{}", query);
        }
        Command::HackerNews { query } => {
            println!("{}", query);
        }
    }
}
