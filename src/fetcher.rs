use rev_buf_reader::RevBufReader;
use std::{io::BufRead, fs::File};

use crate::{model::{Source, LogIndexer}, helper::add_logs_with_thread};

fn lines_from_file(source: Source, log_indexer: &mut LogIndexer) {
    let limit = source.limit as usize;
    let file_path: &str = &source.path;

    let file: File = std::fs::File::open(file_path).expect(&format!("Unable to open file: {}", file_path));
    
    let buf = RevBufReader::new(file);

    let lines = buf.lines().take(limit);

    let mut handles = Vec::new(); 
    let mut batch_data: Vec<String> = Vec::new();
    let mut counter = limit + 1;
    for (i, line) in lines.enumerate() {
        if i >= limit {
            break;
        }
        if line.is_ok() {
            batch_data.push(line.unwrap());
            counter -= 1;

            if counter % 100000 == 0 {
                let index_writer_clone = log_indexer.rwlock_writer.clone();
                let handle = add_logs_with_thread(index_writer_clone, source.id, batch_data, counter, log_indexer.id_field, log_indexer.source_id_field, log_indexer.order_field, log_indexer.log_text_field, log_indexer.log_json_field);
                handles.push(handle);
                batch_data = vec![];
            }
        }
    }
    if !batch_data.is_empty(){
        let index_writer_clone = log_indexer.rwlock_writer.clone();
        let handle = add_logs_with_thread(index_writer_clone, source.id, batch_data, counter, log_indexer.id_field, log_indexer.source_id_field, log_indexer.order_field, log_indexer.log_text_field, log_indexer.log_json_field);
        handles.push(handle);
    }
    
    for handle in handles {
        let _ = handle.join();
    }

    let mut index_writer_wlock = log_indexer.rwlock_writer.write().unwrap();
    let _ = index_writer_wlock.commit();
}

pub fn fetch_data_from_file(source: Source, log_indexer: &mut LogIndexer) {

    lines_from_file(source, log_indexer);
}
