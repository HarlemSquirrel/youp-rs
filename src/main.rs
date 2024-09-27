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
        let res_future = get_response_summary(&args.url_string);

        let response_summary: String = res_future.await?;
        println!("{}", response_summary);

        thread::sleep(five_seconds);
        // thread::sleep(time::Duration::from_millis(10));
    }

    // Ok(())
}

async fn get_response_summary(url_string: &String) -> Result<String, reqwest::Error> {
    let start: chrono::DateTime<chrono::Local> = chrono::offset::Local::now();
    let res_future = reqwest::get(url_string);

    let response: reqwest::Response = res_future.await?;
    let status: reqwest::StatusCode = response.status();
    let now: chrono::DateTime<chrono::Local> = chrono::offset::Local::now();
    let duration: chrono::TimeDelta = now - start;
    let formatted_timestamp = format!("{}", now.format("%Y-%m-%d %H:%M:%S"));

    let colored_status: colored::ColoredString = util::colorize_status(status);
    let response_summary: String = format!(
        "{} Responded {} in {}ms",
        formatted_timestamp,
        colored_status,
        duration.num_milliseconds()
    );

    Ok(response_summary)
}
