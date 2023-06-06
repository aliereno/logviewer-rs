use std::sync::{Mutex, Arc};

use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use tantivy::{IndexWriter, schema::{Schema, Field}, Index};

pub type ArcMutexBackgroundData = Arc<Mutex<BackgroundData>>;

pub struct LogIndexer {
    pub index: Index,
    pub writer: IndexWriter,
    pub schema: Schema,
    pub source_id_field: Field,
    pub log_field: Field,
}


pub struct BackgroundData {
    pub log_indexer: LogIndexer,
    pub sources: Vec<Source>
}


#[derive(SerdeSerialize, SerdeDeserialize, Debug, Clone)]
pub struct Source {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing)]
    pub stats: Option<Stats>,
}

impl Source {
    pub fn new(id: i32, name: String) -> Source {
        Source { id, name, stats: None }
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


#[derive(SerdeSerialize, SerdeDeserialize, Debug, Clone)]
pub struct LogJson {
    id: Option<String>,
    source_id: Option<i32>,
    #[serde(alias = "@timestamp")]
    timestamp: Option<String>,
    app_version: Option<String>,
    request_url: Option<String>,
    request_method: Option<String>,
    request_user_agent: Option<String>,
    request_client_ip: Option<String>,
    request_headers: Option<String>,
    request_by: Option<String>,
    request_body: Option<String>,
    duration: Option<f64>,
    pub response_status_code: Option<i32>,
    response_body: Option<String>,
    exception: Option<String>,
}

impl LogJson {
    
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }
    
    pub fn to_text(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}


#[derive(SerdeSerialize, SerdeDeserialize, Debug, Clone)]
pub struct LogEntry {
    timestamp: String,
    app_name: String,
    endpoint: String,
    pub log: Option<LogJson>,
}

#[derive(SerdeSerialize, SerdeDeserialize, Debug, Clone)]
pub struct Stats {
    pub total_server_errors: usize,
    pub total_client_errors: usize,
    pub total_success_requests: usize,
    pub total_requests: usize,
    pub median_request_time: f32,
    pub max_request_time: f32,
    pub min_request_time: f32,
}

impl LogEntry {
    pub fn from_line(source_id: i32, line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() < 7 {
            return None;
        }
        let timestamp = format!("{} {} {}", parts[0], parts[1], parts[2]);
        let app_name = parts[3].trim_end_matches(':').to_string();
        let endpoint = parts[4].trim_end_matches(':').to_string();
        let json_str = parts[5..].join(" ");
        let mut log: LogJson = serde_json::from_str(&json_str).ok()?;
        log.source_id = Some(source_id);

        Some(LogEntry {
            timestamp,
            app_name,
            endpoint,
            log: Some(log),
        })
    }
}
