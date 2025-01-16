// https://rust-cli.github.io/book/tutorial/cli-args.html
// https://github.com/colored-rs/colored
// https://github.com/seanmonstar/reqwest
// https://async.rs/

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time;

use async_std::task::{self};
use chrono;
use clap::Parser;
use futures::future::join_all;

use reqwest::StatusCode;

mod util;

#[derive(Parser)]
struct Cli {
    /// URL to make the request to
    url_string: String,

    /// Number of workers to make concurrent requests
    #[arg(short, long, default_value_t = 1)]
    concurrency: u8,

    /// Delay in milliseconds between requests in each worker
    #[arg(short, long, default_value_t = 1000)]
    delay: u64,

    /// Number of requests to make from each worker
    #[arg(short, long, default_value_t = 10)]
    iterations: u64,
}

#[derive(Copy, Clone)]
struct StatusCodeStats {
    num_requests: u16,
    avg_duration_ms: i64,
    max_duration_ms: i64,
}

type SharableStats = Arc<Mutex<HashMap<StatusCode, StatusCodeStats>>>;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let iterations = args.iterations;
    let throttle: time::Duration = time::Duration::from_millis(args.delay);
    let sharable_stats = Arc::new(Mutex::new(HashMap::new()));

    println!("URL: {:?}", args.url_string);

    let mut loops = vec![];
    for index in 0..usize::from(args.concurrency) {
        // Clone the handle to the hash map.
        let sharable_stats_clone = sharable_stats.clone();
        let url_string = args.url_string.clone();
        loops.insert(
            index,
            tokio::spawn(async move {
                check_loop(&url_string, iterations, sharable_stats_clone, throttle).await;
            })
        );
    }
    join_all(loops).await;

    println!("\nResults by status code:");
    let stats = sharable_stats.lock().unwrap();
    let mut total_count = 0;
    for (status_code, status_code_stats) in stats.clone().into_iter() {
        let count = status_code_stats.num_requests;
        total_count = total_count + count;
        let colorized_status = util::colorize_status(status_code);
        let avg_duration_ms = status_code_stats.avg_duration_ms;
        let max_duration_ms = status_code_stats.max_duration_ms;
        println!("{colorized_status}: {count} (avg {avg_duration_ms}ms, max {max_duration_ms}ms)");
    }
    println!("Total: {}", total_count);
}

async fn check_loop(url_string: &String, num_iterations:u64, stats: SharableStats, throttle: time::Duration) {
    for i in 0..num_iterations {
        let stats = stats.clone();
        let url_string = url_string.clone();
        let _result = tokio::spawn(async move {
            print_response_for(&url_string, stats).await;
        }).await;
        if i < (num_iterations - 1) {
          task::sleep(throttle).await;
        }
    }
}

async fn print_response_for(url_string: &String, stats: SharableStats) {
    let res_future = get_response_summary(&url_string, stats);

    match res_future.await {
        Ok(response_summary) => println!("{}", response_summary),
        Err(e) => println!("Yikes! {}", e),
    }
}

async fn get_response_summary(url_string: &String, stats: SharableStats) -> Result<String, reqwest::Error> {
    let start: chrono::DateTime<chrono::Local> = chrono::offset::Local::now();
    let res_future = reqwest::get(url_string);

    let response: reqwest::Response = res_future.await?;
    let status: reqwest::StatusCode = response.status();
    let now: chrono::DateTime<chrono::Local> = chrono::offset::Local::now();
    let duration: chrono::TimeDelta = now - start;
    let formatted_timestamp = format!("{}", now.format("%Y-%m-%d %H:%M:%S"));

    let mut stats = stats.lock().unwrap();
    let default_status_code_stats = StatusCodeStats {
        num_requests: 0,
        avg_duration_ms: 0,
        max_duration_ms: 0,
    };
    // let existing_count = stats.get(&status).copied().unwrap_or(0);
    let mut status_code_stats = stats.get(&status).copied().unwrap_or(default_status_code_stats);
    status_code_stats.avg_duration_ms =
        (duration.num_milliseconds() + (status_code_stats.avg_duration_ms * i64::from(status_code_stats.num_requests))) / (i64::from(status_code_stats.num_requests + 1));
    status_code_stats.num_requests = status_code_stats.num_requests + 1;
    if duration.num_milliseconds() > status_code_stats.max_duration_ms {
        status_code_stats.max_duration_ms = duration.num_milliseconds();
    }

    stats.insert(status, status_code_stats);

    let colored_status: colored::ColoredString = util::colorize_status(status);
    let response_summary: String = format!(
        "{} Responded {} in {}ms",
        formatted_timestamp,
        colored_status,
        duration.num_milliseconds()
    );

    Ok(response_summary)
}
