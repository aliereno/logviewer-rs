use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PageOut<T> {
    pub current_page: usize,
    pub total_pages: usize,
    pub items: Option<Vec<T>>,
    pub total_count: usize,
}

#[derive(Deserialize)]
pub struct PageFilterIn {
    pub current_page: Option<usize>,
    pub page_size: Option<usize>,
    pub search: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct StatOut {
    pub ram_usage: f64,
    pub queue_count: i64,
}