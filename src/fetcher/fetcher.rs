use std::{io::BufRead};
use core::time::Duration;
use actix_web::rt::time;

use crate::model::{LogEntry, Stats, ArcMutexBackgroundData};

pub fn parse_log_file(file_path: &str) -> Vec<LogEntry> {
    let file = std::fs::File::open(file_path).expect("Unable to open file");
    let reader = std::io::BufReader::new(file);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| LogEntry::from_line(&line))
        .collect()
}

pub fn fetch_statistics() -> Stats {
    let entries = parse_log_file("logs/example.log");

    let mut stats = Stats {
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
                    stats.total_server_errors += 1;
                } else if log.response_status_code.unwrap_or_default() >= 400 {
                    stats.total_client_errors += 1;
                } else if log.response_status_code.unwrap_or_default() >= 200 {
                    stats.total_success_requests += 1;
                }
            }
            _ => {}
        }
    }
    stats.total_requests =
        stats.total_client_errors + stats.total_server_errors + stats.total_success_requests;

    return stats;
}


pub async fn run_background_task(shared_data: ArcMutexBackgroundData) {
    let mut interval = time::interval(Duration::from_secs(10));

    loop {

        let mut data = shared_data.lock().unwrap();
        println!("hello");

        interval.tick().await;

    }
}