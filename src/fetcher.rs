use actix_web::rt::time;
use rev_buf_reader::RevBufReader;
use core::time::Duration;
use std::{io::BufRead, fs::File};

use crate::model::{ArcMutexBackgroundData, Source};

fn lines_from_file(file_path: &str, limit: usize) -> Vec<String> {
    let file: File = std::fs::File::open(file_path).expect(&format!("Unable to open file: {}", file_path));
    let buf = RevBufReader::new(file);
    buf.lines().take(limit).filter_map(|line| line.ok()).collect()
}

pub fn fetch_data_from_file(source: Source) -> Vec<String> {
    let file_path = &source.path;

    let logs = lines_from_file(file_path, 1000);
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
            let _ = data.log_indexer.reset_indexes_by_source_id(source.clone());
        }

        println!("finished");
    }
}
