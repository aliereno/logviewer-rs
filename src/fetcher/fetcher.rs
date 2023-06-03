use std::{io::BufRead};
use core::time::Duration;
use actix_web::rt::time;

use crate::model::{LogEntry, Stats, ArcMutexBackgroundData, LogJson};

pub fn parse_log_file(file_path: &str) -> Vec<LogEntry> {
    let file = std::fs::File::open(file_path).expect("Unable to open file");
    let reader = std::io::BufReader::new(file);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| LogEntry::from_line(&line))
        .collect()
}

pub fn fetch_data_from_file(file_path: String) -> (Option<Stats>, Option<Vec<LogJson>>) {
    let entries = parse_log_file(&file_path);
    println!("readed file {} lines {}", file_path, entries.len());
    let mut stats = Stats {
        total_server_errors: 0,
        total_client_errors: 0,
        total_success_requests: 0,
        total_requests: 0,
        median_request_time: 0.0,
        max_request_time: 0.0,
        min_request_time: 0.0,
    };

    let mut logs: Vec<LogJson> = vec![];

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
                logs.push(log);
            }
            _ => {}
        }
    }
    stats.total_requests =
        stats.total_client_errors + stats.total_server_errors + stats.total_success_requests;

    return (Some(stats), Some(logs));
}

pub async fn run_background_task(shared_data: ArcMutexBackgroundData) {
    let mut interval = time::interval(Duration::from_secs(180));

    loop {
        interval.tick().await;
        println!("started");

        let mut data = shared_data.lock().unwrap();

        for source in &mut data.sources {
            let (stats, logs) = fetch_data_from_file(source.name.clone());
            source.set_stats(stats);
            source.set_logs(logs);
        }

        println!("finished");

    }
}