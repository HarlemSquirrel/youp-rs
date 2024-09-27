// https://rust-cli.github.io/book/tutorial/cli-args.html
// https://github.com/colored-rs/colored
// https://github.com/seanmonstar/reqwest

use std::{thread, time};

use chrono;
use clap::Parser;

mod util;

#[derive(Parser)]
struct Cli {
    url_string: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let five_seconds = time::Duration::from_secs(5);
    let args = Cli::parse();

    println!("URL: {:?}", args.url_string);

    loop {
        let start: chrono::DateTime<chrono::Local> = chrono::offset::Local::now();
        let res_future = reqwest::get(&args.url_string);
        let response: reqwest::Response = reqwest::get(&args.url_string)
            .await?;
        let now: chrono::DateTime<chrono::Local> = chrono::offset::Local::now();
        let status: reqwest::StatusCode = response.status();
        let duration: chrono::TimeDelta = now - start;
        let formatted_timestamp = format!("{}", now.format("%Y-%m-%d %H:%M:%S"));

        let colored_status: colored::ColoredString = util::colorize_status(status);
        println!("{} Responded {} in {}ms", formatted_timestamp, colored_status, duration.num_milliseconds());

        thread::sleep(five_seconds);
    }

    // Ok(())
}
