use axum::extract::Path;
use axum::response::{Json, IntoResponse};

use super::entities::InsertItemReq;
use super::usecases;

pub async fn find_items() -> impl IntoResponse {
    usecases::find_items().await
}

pub async fn find_one_item(Path(item_id): Path<String>) -> impl IntoResponse {
    usecases::find_one_item(item_id).await
}

pub async fn insert_one_item(Json(req): Json<InsertItemReq>) -> impl IntoResponse {
    usecases::insert_one_item(req).await
}

pub async fn update_one_item(Path(item_id): Path<String>, Json(req): Json<InsertItemReq>) -> impl IntoResponse {
    usecases::update_one_item(item_id, req).await
}

pub async fn delete_one_item(Path(item_id): Path<String>) -> impl IntoResponse {
    usecases::delete_one_item(item_id).await
}