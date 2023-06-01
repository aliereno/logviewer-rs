
use std::io::BufRead;

use super::model::{LogEntry, Stats};


pub fn parse_log_file(file_path: &str) -> Vec<LogEntry> {
    let file = std::fs::File::open(file_path).expect("Unable to open file");
    let reader = std::io::BufReader::new(file);
    reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| LogEntry::from_line(&line))
        .collect()
}

pub fn fetch_statistics() -> Stats {
    let entries = parse_log_file("logs/example.log");

    let mut stats = Stats { 
        _server_errors: vec![], 
        _client_errors: vec![], 
        _success_requests: vec![], 
        total_server_errors: 0, 
        total_client_errors: 0, 
        total_success_requests: 0, 
        total_requests: 0, 
        median_request_time: 0.0, 
        max_request_time: 0.0, 
        min_request_time: 0.0, 
    };

    for entry in entries {
        match entry.log {
            Some(log) => {
                
                if log.response_status_code.unwrap_or_default() >= 500 {
                    stats._server_errors.push(log);
                }else if log.response_status_code.unwrap_or_default() >= 400 {
                    stats._client_errors.push(log);
                }else if log.response_status_code.unwrap_or_default() >= 200 {
                    stats._success_requests.push(log);
                }

            },
            _ => {},
        }
    }
    stats.total_client_errors = stats._client_errors.len();
    stats.total_server_errors = stats._server_errors.len();
    stats.total_success_requests = stats._success_requests.len();
    stats.total_requests = stats.total_client_errors + stats.total_server_errors + stats.total_success_requests;
    
    return stats;
}
