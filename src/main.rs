// https://rust-cli.github.io/book/tutorial/cli-args.html
// https://github.com/colored-rs/colored
// https://github.com/seanmonstar/reqwest
// https://async.rs/

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time;

use async_std::task::{self};
// use chrono;
use clap::Parser;
use futures::future::join_all;

// use reqwest::StatusCode;

mod req;
mod stats;
mod util;

use crate::stats::SharableStats;
use crate::req::get_response_summary;

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
    // Early validation of URL
    if let Err(e) = url::Url::parse(url_string) {
        println!("Please provide a valid URL. Error: {}", e);
        return;
    }
    
    for i in 0..num_iterations {
        let stats = stats.clone();
        let url_string = url_string.clone();
        let _result = tokio::spawn(async move {
            let _ = print_response_for(&url_string, stats).await;
        }).await;
        if i < (num_iterations - 1) {
          task::sleep(throttle).await;
        }
    }
}

async fn print_response_for(url_string: &String, stats: SharableStats) -> Result<(), String> {
    // Validate URL early to avoid running all iterations for invalid URLs
    let url_result = url::Url::parse(url_string);
    if let Err(e) = url_result {
        return Err(format!("Please provide a valid URL. Error: {}", e));
    }

    let res_future = get_response_summary(&url_string, stats);

    match res_future.await {
        Ok(response_summary) => println!("{}", response_summary),
        Err(e) => {
            // Check if this is a URL parsing error and provide a more helpful message
            let error_msg = e.to_string();
            if error_msg.contains("builder error") || error_msg.contains("invalid url") {
                println!("Please provide a valid URL");
            } else {
                println!("Yikes! {}", e);
            }
        },
    }
    
    Ok(())
}
