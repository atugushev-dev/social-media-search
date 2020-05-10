use reqwest::blocking::Client;

#[derive(Debug)]
pub struct HackerNews {}

impl HackerNews {
    pub fn new() -> HackerNews {
        HackerNews {}
    }
    pub fn search(&self, client: &Client, query: &str, days: u32) {
        println!("Hackernews stub");
    }
}
