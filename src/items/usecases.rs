use axum::{
    response::{Json, IntoResponse},
    http::StatusCode,
};
use serde_json::json;
use bson::oid::ObjectId;
use tracing::info;

use super::entities::{Item, Result, InsertItemReq, ItemBson};
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
                    "message": "insert item failed",
                })).into_response()
            )
        }
    };

    match repositories::find_one_item(item_id).await {
        Result::Ok(item) =>  (
            StatusCode::CREATED,
            Json(item).into_response()
        ),
        Result::Err(e) => {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
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
                    "message": e,
                })).into_response()
            )
        }
    }
}

pub async fn update_one_item(item_id: String, req: InsertItemReq) -> impl IntoResponse {
    let item_object_id = match ObjectId::parse_str(item_id) {
        Ok(id) => id,
        Err(_) => return {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": "parse ObjectId failed",
                })).into_response()
            )
        }
    };

    let item_to_update = ItemBson {
        _id: item_object_id,
        name: req.name,
        description: req.description,
        damage: req.damage,
        level_required: req.level_required,
        price: req.price
    };

    match repositories::update_one_item(item_to_update).await {
        Result::Ok(r) => info!("{:?}", r),
        Result::Err(_) => return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "update one item failed",
            })).into_response()
        )
    }

    match repositories::find_one_item(item_object_id).await {
        Result::Ok(item) =>  (
            StatusCode::OK,
            Json(item).into_response()
        ),
        Result::Err(e) => {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": e,
                })).into_response()
            )
        }
    }
}

pub async fn delete_one_item(item_id: String) -> impl IntoResponse {
    let item_object_id = match ObjectId::parse_str(item_id) {
        Ok(id) => id,
        Err(e) => {
            info!("Error: delete one item failed: {:?}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": "delete one item failed",
                })).into_response()
            )
        }
    };

    match repositories::delete_one_item(item_object_id).await {
        Result::Ok(r) => (
            StatusCode::OK,
            Json(json!(r)).into_response()
        ),
        Result::Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "delete one item failed",
            })).into_response()
        )
    }
}