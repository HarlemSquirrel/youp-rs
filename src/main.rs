// https://rust-cli.github.io/book/tutorial/cli-args.html
// https://github.com/colored-rs/colored
// https://github.com/seanmonstar/reqwest

use std::{thread, time};

use chrono;
use colored::Colorize;
use clap::Parser;

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
        let status = reqwest::get(&args.url_string)
            .await?
            .status();

        let now = chrono::offset::Local::now();
        let colored_status = match status.as_u16() {
            200..=299 => status.to_string().green(),
            400..=499 => status.to_string().yellow(),
            500..=599 => status.to_string().red(),
            _ => status.to_string().normal(),
        };
        println!("{} Response status: {}", now, colored_status);
        thread::sleep(five_seconds);
    }

    // Ok(())
}
