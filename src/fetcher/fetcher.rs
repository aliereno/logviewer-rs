use std::{io::BufRead};
use core::time::Duration;
use actix_web::rt::time;

use crate::model::{ArcMutexBackgroundData, Source};

pub fn parse_log_file(file_path: &str) -> Vec<String> {
    let file = std::fs::File::open(file_path).expect("Unable to open file");
    let reader = std::io::BufReader::new(file);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .collect()
}

pub fn fetch_data_from_file(source: Source) -> Vec<String> {
    let file_path = &source.name;

    let logs = parse_log_file(file_path);
    println!("readed file {} lines {}", file_path, logs.len());
    
    return logs;
}

pub async fn run_background_task(shared_data: ArcMutexBackgroundData) {
    let mut interval = time::interval(Duration::from_secs(1800));

    loop {
        interval.tick().await;
        println!("started");

        let mut data = shared_data.lock().unwrap();

        for source in &mut data.sources.clone() {
            let logs = fetch_data_from_file(source.clone());

            match data.log_indexer.add_logs(source.id, &logs) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            }   
        }
            
        println!("finished");

    }
}