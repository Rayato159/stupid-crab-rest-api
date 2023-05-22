use serde::Serialize;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Serialize)]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub description: String,
}