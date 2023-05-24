use serde::{Serialize, Deserialize};
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub description: String,
}