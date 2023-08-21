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

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct InsertProductReq {
    pub title: String,
    pub description: String,
}

impl Product {
    pub fn new() -> Product {
        Product {
            id: 0,
            title: String::from(""),
            description: String::from(""),
        }
    }
}