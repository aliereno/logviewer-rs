
use regex::Regex;
use tantivy::{doc, Document, IndexWriter};

use lazy_static::lazy_static;
use std::{thread::{JoinHandle, self}, sync::{Arc, RwLock}};

use crate::model::{IndexFields, RwLockStat};

lazy_static! {
    static ref JSON_REGEX: Regex =
        // TODO: compare 
        // Regex::new(r#"\{.*"#).unwrap();
        Regex::new(r#"(?s)\{([^{}]*(?:\{[^{}]*}[^{}]*)*)}"#).unwrap();
}


pub fn log_to_document(source_id: i32, log: String, index: usize, fields: IndexFields) -> Document {

    let mut doc = Document::new();
    doc.add_text(fields.id_field, format!("{}#{}", source_id, index));
    doc.add_i64(fields.source_id_field, source_id.into());
    doc.add_i64(fields.order_field, index as i64);
    doc.add_text(fields.log_text_field, log.clone());

    // TODO: find faster solutions for json parsing
    let matched_text = JSON_REGEX.find(&log);

    match matched_text {
        Some(m) => {
            if let Ok(key) = serde_json::from_str(m.as_str()) {
                doc.add_json_object(
                    fields.log_json_field,
                    key,
                );
            }
        },
        None => ()
    }

    doc
}



pub fn add_logs_with_thread(rwlock_writer: Arc<RwLock<tantivy::IndexWriter>>, source_id: i32, logs: Vec<String>, start_index: usize, fields: IndexFields) -> JoinHandle<()> {

    let index_writer_clone = rwlock_writer;
    
    thread::spawn(move || {
        let index_writer_rlock = index_writer_clone.read().unwrap();
        for (index, log) in logs.iter().rev().enumerate() {
            let doc = log_to_document(source_id, log.to_string(), start_index + index, fields.clone());
            let _ = index_writer_rlock.add_document(doc);
        };
    })
}


pub fn commit_on_index_writer(rwlock_writer: Arc<RwLock<IndexWriter>>) {
    let mut index_writer_wlock = rwlock_writer.write().unwrap();
    let _ = index_writer_wlock.commit();
}


pub fn update_stat(stat: RwLockStat, new_value: i64) {
    let mut writer = stat.write().unwrap();
    writer.queue_count += new_value;
}
