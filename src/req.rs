use chrono;

use crate::stats::SharableStats;
use crate::stats::StatusCodeStats;
use crate::util::colorize_status;

pub async fn get_response_summary(url_string: &String, stats: SharableStats) -> Result<String, reqwest::Error> {
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

    let colored_status: colored::ColoredString = colorize_status(status);
    let response_summary: String = format!(
        "{} Responded {} in {}ms",
        formatted_timestamp,
        colored_status,
        duration.num_milliseconds()
    );

    Ok(response_summary)
}
