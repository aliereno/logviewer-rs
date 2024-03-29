use rev_buf_reader::RevBufReader;
use std::{io::BufRead, fs::File};

use crate::{model::{Source, IndexFields, RwLockIndexWriter}, helper::{add_logs_with_thread, commit_on_index_writer}};


pub fn fetch_data_from_file(source: Source, index_writer: &mut RwLockIndexWriter, fields: IndexFields) {
    let limit = source.limit as usize;
    let file_path: &str = &source.path;

    let file: File = std::fs::File::open(file_path).unwrap_or_else(|_| panic!("Unable to open file: {}", file_path));
    
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
                let index_writer_clone = index_writer.clone();
                let handle = add_logs_with_thread(index_writer_clone, source.id, batch_data, counter, fields.clone());
                handles.push(handle);
                batch_data = vec![];
            }
        }
    }
    if !batch_data.is_empty(){
        let index_writer_clone = index_writer.clone();
        let handle = add_logs_with_thread(index_writer_clone, source.id, batch_data, counter, fields);
        handles.push(handle);
    }
    
    for handle in handles {
        let _ = handle.join();
    }

    commit_on_index_writer(index_writer.clone());
}
