use std::sync::{Mutex, Arc};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use tantivy::{IndexWriter, schema::{Schema, Field}, Index};

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
    pub sources: Vec<Source>
}


#[derive(SerdeSerialize, SerdeDeserialize, Debug, Clone)]
pub struct Source {
    pub id: i32,
    pub name: String,
}

impl Source {
    pub fn new(id: i32, name: String) -> Source {
        Source { id, name }
    }

    pub fn from_env(env_string: String) -> Vec<Source> {
        let mut result = vec![];

        let index: i32 = 1; 
        for splitted in env_string.split(",") {
            result.push(Source::new(index, splitted.into()));
        }

        result
    }
}