use axum::{
    response::{Json, IntoResponse, Response},
    extract::{Path},
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub description: String,
}

pub async fn find_one_product(Path(product_id): Path<i32>) -> Response {
    let p: Product = Product {
        id: 1,
        title: String::from("Minecraft"),
        description: String::from("Java Edition"),
    };

    if product_id == 1 {
        return Json(p).into_response()
    }

    Json(json!({
        "status": 200,
        "message": format!("product_id: {} not found", product_id),
    })).into_response()
}