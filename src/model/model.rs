use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tantivy::{
    schema::{Field, Schema},
    Index, IndexWriter,
};

pub type ArcMutexBackgroundData = Arc<Mutex<BackgroundData>>;

pub struct LogIndexer {
    pub index: Index,
    pub writer: IndexWriter,
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
}

impl Source {
    pub fn new(id: i32, name: String, path: String) -> Source {
        Source { id, name, path }
    }

    pub fn from_config(config: AppConfig) -> Vec<Source> {
        let mut result = vec![];

        let mut index: i32 = 1;
        for log_config in config.logs {
            result.push(Source::new(index, log_config.name, log_config.path));
            index += 1;
        }

        result
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub index_dir: String,
    pub logs: Vec<LogConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    pub name: String,
    pub path: String,
}
