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
        let status = reqwest::get(&args.url_string)
            .await?
            .status();

        let now = chrono::offset::Local::now();
        let colored_status = util::colorize_status(status);
        println!("{} Response status: {}", now, colored_status);
        thread::sleep(five_seconds);
    }

    // Ok(())
}
