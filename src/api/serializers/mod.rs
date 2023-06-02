use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PageOut<T> {
    pub current_page: i32,
    pub total_page: i32,
    pub items: Option<Vec<T>>,
}
