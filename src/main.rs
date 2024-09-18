// https://rust-cli.github.io/book/tutorial/cli-args.html
// https://github.com/colored-rs/colored
// https://github.com/seanmonstar/reqwest

use std::{thread, time};

use chrono;
use colored::{ColoredString, Colorize};
use clap::Parser;
use reqwest::StatusCode;

#[derive(Parser)]
struct Cli {
    url_string: String,
}

fn colorized_status(status: StatusCode) -> ColoredString {
    return match status.as_u16() {
        200..=299 => status.to_string().green(),
        400..=499 => status.to_string().yellow(),
        500..=599 => status.to_string().red(),
        _ => status.to_string().normal(),
    };
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
        let colored_status = colorized_status(status);
        println!("{} Response status: {}", now, colored_status);
        thread::sleep(five_seconds);
    }

    // Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colorized_status() {
        assert_eq!(colorized_status(StatusCode::OK), "200 OK".green());
        assert_eq!(colorized_status(StatusCode::NOT_FOUND), "404 Not Found".yellow());
        assert_eq!(colorized_status(StatusCode::INTERNAL_SERVER_ERROR), "500 Internal Server Error".red());
    }
}
