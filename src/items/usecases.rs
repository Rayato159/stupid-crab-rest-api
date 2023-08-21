use axum::{
    response::{Json, IntoResponse},
    http::StatusCode,
};
use serde_json::json;
use bson::oid::ObjectId;

use super::entities::{Item, Result, InsertItemReq};
use super::repositories;

pub async fn find_items() -> impl IntoResponse {
    let items: Vec<Item> = repositories::find_items().await;
    (StatusCode::OK, Json(items).into_response())
}

pub async fn insert_one_item(req: InsertItemReq) -> impl IntoResponse {
    let result_object_id = repositories::insert_one_item(req).await;
    let item_id: ObjectId = match result_object_id {
        Result::Ok(id) => id,
        Result::Err(_) => return {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": 400,
                    "message": "insert item failed",
                })).into_response()
            )
        }
    };

    match repositories::find_one_item(item_id).await {
        Result::Ok(item) =>  (
            StatusCode::OK,
            Json(item).into_response()
        ),
        Result::Err(e) => {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": 400,
                    "message": e,
                })).into_response()
            )
        }
    }
}

pub async fn find_one_item(item_id: String) -> impl IntoResponse {
    let item_object_id = match ObjectId::parse_str(item_id) {
        Ok(id) => id,
        Err(_) => return {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": 400,
                    "message": "parse ObjectId failed",
                })).into_response()
            )
        }
    };

    match repositories::find_one_item(item_object_id).await {
        Result::Ok(item) =>  (
            StatusCode::OK,
            Json(item).into_response()
        ),
        Result::Err(e) => {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": 400,
                    "message": e,
                })).into_response()
            )
        }
    }
}

// pub fn update_product(req: Item) -> Result<Item, String> {
//     let products = products_db();

//     for mut product in products {
//         if req.id == product.id {
//             if req.title != String::from("") {
//                 product.title = req.title
//             }
//             if req.description != String::from("") {
//                 product.description = req.description
//             }
//             return Result::Ok(product)
//         }
//     }
//     Result::Err(format!("product_id {} not found", req.id))
// }

// pub fn delete_product(product_id: i32) -> Result<Vec<Item>, String> {
//     let mut products = products_db();

//     for (i, product) in products.iter().enumerate() {
//         if product_id == product.id {
//             products.remove(i);
//             return Result::Ok(products)
//         }
//     }
//     Result::Err(format!("product_id {} not found", product_id))
// }