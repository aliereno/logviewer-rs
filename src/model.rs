use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex, RwLock};
use tantivy::{
    schema::{Field, Schema},
    Index,
};

pub type ArcMutexBackgroundData = Arc<Mutex<BackgroundData>>;
pub type RwLockIndexWriter= Arc<RwLock<tantivy::IndexWriter>>;
pub struct LogIndexer {
    pub index: Index,
    pub rwlock_writer: RwLockIndexWriter,
    pub schema: Schema,
    pub id_field: Field,
    pub source_id_field: Field,
    pub order_field: Field,
    pub log_text_field: Field,
    pub log_json_field: Field,
}

pub struct BackgroundData {
    pub log_indexer: LogIndexer,
    pub sources: Vec<Source>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub limit: i32,
}

impl Source {
    pub fn new(id: i32, name: String, path: String, limit: Option<i32>) -> Source {
        Source { id, name, path, limit: limit.unwrap_or(1000) }
    }

    pub fn from_config(config: AppConfig) -> Vec<Source> {
        let mut result = vec![];

        let mut index: i32 = 1;
        for log_config in config.logs {
            result.push(Source::new(index, log_config.name, log_config.path, log_config.limit));
            index += 1;
        }

        result
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub index_dir: String,
    pub logs: Vec<LogConfig>,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    pub name: String,
    pub path: String,
    pub limit: Option<i32>
}
