
use regex::Regex;
use tantivy::{doc, Document, schema::Field};

use lazy_static::lazy_static;
use std::thread::{JoinHandle, self};

use crate::model::RwLockIndexWriter;

lazy_static! {
    static ref JSON_REGEX: Regex =
        Regex::new(r#"(?s)\{([^{}]*(?:\{[^{}]*}[^{}]*)*)}"#).unwrap();
}


pub fn log_to_document(source_id: i32, log: String, index: usize, id_field: Field, source_id_field: Field, order_field: Field, log_text_field: Field, log_json_field: Field) -> Document {
    // TODO: find faster solutions for json parsing

    let matches: Vec<_> = JSON_REGEX
        .captures_iter(&log)
        .map(|caps| caps.get(0).unwrap())
        .collect();
    let modified_log: String = JSON_REGEX.replace_all(&log, "{{JSON}}").to_string();

    let mut doc = Document::new();
    doc.add_text(id_field, format!("{}#{}", source_id, index));
    doc.add_i64(source_id_field, source_id.into());
    doc.add_i64(order_field, index as i64);
    doc.add_text(log_text_field, modified_log);

    for matched_text in matches {
        if let Ok(key) = serde_json::from_str(matched_text.as_str()) {
            doc.add_json_object(
                log_json_field,
                key,
            );
        }
    }

    doc
}



pub fn add_logs_with_thread(rwlock_writer: RwLockIndexWriter, source_id: i32, logs: Vec<String>, start_index: usize, id_field: Field,
    source_id_field: Field,
    order_field: Field,
    log_text_field: Field,
    log_json_field: Field) -> JoinHandle<()> {

    let index_writer_clone = rwlock_writer.clone();
    let handle = thread::spawn(move || {
        let _opstamp = {
            let index_writer_rlock = index_writer_clone.read().unwrap();
            for (index, log) in logs.iter().rev().enumerate() {
                let doc = log_to_document(source_id, log.to_string(), start_index + index, id_field, source_id_field, order_field, log_text_field, log_json_field);
                let _ = index_writer_rlock.add_document(doc);
            }
        };
    });

    return handle;
}
