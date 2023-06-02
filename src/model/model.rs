use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogJson {
    #[serde(alias = "@timestamp")]
    timestamp: Option<String>,
    app_version: Option<String>,
    request_url: Option<String>,
    request_method: Option<String>,
    request_user_agent: Option<String>,
    request_client_ip: Option<String>,
    request_headers: Option<String>,
    request_by: Option<String>,
    request_body: Option<String>,
    duration: Option<f64>,
    pub response_status_code: Option<i32>,
    response_body: Option<String>,
    exception: Option<String>,
}
impl LogJson {
    pub fn get_dummy_vec() -> Vec<LogJson> {
        let dummy_item = LogJson {
            timestamp: Some("2023-05-30T12:59:08.200028".to_string()),
            app_version: Some("0.0.1".to_string()),
            request_url: Some("http://0.0.0.0:8000/docs".to_string()),
            request_method: Some("2023-05-30T12:59:08.200028".to_string()),
            request_user_agent: Some("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36".to_string()),
            request_client_ip: Some("127.0.0.1".to_string()),
            request_headers: Some("".to_string()),
            request_by: Some("".to_string()),
            request_body: Some("".to_string()),
            duration: Some(0.010),
            response_status_code: Some(200),
            response_body: Some("".to_string()),
            exception: None,
        };

        vec![dummy_item.clone(), dummy_item.clone(), dummy_item.clone()]
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntry {
    timestamp: String,
    app_name: String,
    endpoint: String,
    pub log: Option<LogJson>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub total_server_errors: usize,
    pub total_client_errors: usize,
    pub total_success_requests: usize,
    pub total_requests: usize,
    pub median_request_time: f32,
    pub max_request_time: f32,
    pub min_request_time: f32,
}

impl LogEntry {
    pub fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() < 7 {
            return None;
        }
        let timestamp = format!("{} {} {}", parts[0], parts[1], parts[2]);
        let app_name = parts[3].trim_end_matches(':').to_string();
        let endpoint = parts[4].trim_end_matches(':').to_string();
        let json_str = parts[5..].join(" ");
        let log: LogJson = serde_json::from_str(&json_str).ok()?;

        Some(LogEntry {
            timestamp,
            app_name,
            endpoint,
            log: Some(log),
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    id: i32,
    name: String,
}

impl Source {
    pub fn new(id: i32, name: String) -> Source {
        Source { id, name }
    }
}
