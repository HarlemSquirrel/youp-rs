use colored::{ColoredString, Colorize};
use reqwest::StatusCode;

pub fn colorize_status(status: StatusCode) -> ColoredString {
  return match status.as_u16() {
      200..=299 => status.to_string().green(),
      400..=499 => status.to_string().yellow(),
      500..=599 => status.to_string().red(),
      _ => status.to_string().normal(),
  };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colorize_status() {
        assert_eq!(colorize_status(StatusCode::OK), "200 OK".green());
        assert_eq!(colorize_status(StatusCode::NOT_FOUND), "404 Not Found".yellow());
        assert_eq!(colorize_status(StatusCode::INTERNAL_SERVER_ERROR), "500 Internal Server Error".red());
    }
}
