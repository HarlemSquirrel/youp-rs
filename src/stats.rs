use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use reqwest::StatusCode;

#[derive(Copy, Clone)]
pub struct StatusCodeStats {
    pub num_requests: u16,
    pub avg_duration_ms: i64,
    pub max_duration_ms: i64,
}

pub type SharableStats = Arc<Mutex<HashMap<StatusCode, StatusCodeStats>>>;
