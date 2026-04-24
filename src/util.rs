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
    
    #[test]
    fn test_url_parsing() {
        // Test that valid URLs parse correctly
        let valid_urls = vec![
            "https://httpbin.org/get",
            "http://example.com",
            "https://github.com",
        ];
        
        for url in valid_urls {
            assert!(url::Url::parse(url).is_ok(), "Valid URL should parse: {}", url);
        }
        
        // Test that invalid URLs fail appropriately
        let invalid_urls = vec![
            "test", 
            "http://",
            "not-a-url"
        ];
        
        for url in invalid_urls {
            assert!(url::Url::parse(url).is_err(), "Invalid URL should fail to parse: {}", url);
        }
    }
}
