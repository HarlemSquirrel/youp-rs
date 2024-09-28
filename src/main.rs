// https://rust-cli.github.io/book/tutorial/cli-args.html
// https://github.com/colored-rs/colored
// https://github.com/seanmonstar/reqwest
// https://async.rs/

use std::time;
use async_std::task;

use chrono;
use clap::Parser;

use futures::future::join_all;

mod util;

#[derive(Parser)]
struct Cli {
    url_string: String,

    #[arg(short, long, default_value_t = 1)]
    concurrency: u8,
}

#[tokio::main]
async fn main() {
    let throttle: time::Duration = time::Duration::from_secs(5);
    let args = Cli::parse();

    println!("URL: {:?}", args.url_string);

    let mut loops = vec![];
    for index in 0..usize::from(args.concurrency) {
        loops.insert(index, check_loop(&args.url_string, throttle));
    }
    join_all(loops).await;
}

async fn check_loop(url_string: &String, throttle: time::Duration) {
    for _n in 0..1000 {
        print_response_for(&url_string).await;
        task::sleep(throttle).await;
    }
}

async fn print_response_for(url_string: &String) {
    let res_future = get_response_summary(&url_string);

    match res_future.await {
        Ok(response_summary) => println!("{}", response_summary),
        Err(e) => println!("Yikes! {}", e),
    }
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
