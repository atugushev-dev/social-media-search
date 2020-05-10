mod hackernews;
mod stackoverflow;

extern crate chrono;
extern crate reqwest;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(short, long)]
    query: String,

    #[structopt(short, long, required = false, default_value = "1")]
    days: u32,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "social-media-search")]
enum Command {
    /// Stack Overflow
    #[structopt(name = "so")]
    StackOverflow,
    /// Twitter
    #[structopt(name = "tw")]
    Twitter,
    /// Hacker News
    #[structopt(name = "hn")]
    HackerNews,
}

fn main() {
    let client = reqwest::blocking::Client::builder()
        .gzip(true)
        .build()
        .expect("failed to build client");

    let opts = Opts::from_args();

    match opts.cmd {
        Command::StackOverflow => {
            stackoverflow::StackOverflow::new().search(&client, &opts.query, opts.days)
        }
        Command::HackerNews => {
            hackernews::HackerNews::new().search(&client, &opts.query, opts.days)
        }
        _ => (),
    }
}
