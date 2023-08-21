use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Item {
    pub _id: String,
    pub name: String,
    pub description: String,
    pub damage: i32,
    pub level_required: i32,
    pub price: i32
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct ItemBson {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub damage: i32,
    pub level_required: i32,
    pub price: i32
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct InsertItemReq {
    pub name: String,
    pub description: String,
    pub damage: i32,
    pub level_required: i32,
    pub price: i32
}

impl Item {
    pub fn new() -> Item {
        Item {
            _id: String::from(""),
            name: String::from(""),
            description: String::from(""),
            damage: 0,
            level_required: 0,
            price: 0
        }
    }
}