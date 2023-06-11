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
