use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PageOut<T> {
    pub current_page: i32,
    pub total_page: i32,
    pub items: Option<Vec<T>>,
}


#[derive(Deserialize)]
pub struct PageFilterIn {
    pub current_page: Option<i32>,
    pub total_page: Option<i32>,
    pub search: Option<String>,
}