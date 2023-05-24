use axum::{
    response::{Json, IntoResponse},
    extract::{Path},
    http::{StatusCode},
};
use serde_json::json;

use super::entities::{Product, Result};
use super::usecases;

pub async fn find_one_product(Path(product_id): Path<i32>) -> impl IntoResponse {
    let product_id_int = product_id;
    match usecases::find_one_product(product_id_int) {
        Result::Ok(product) => {
            (StatusCode::OK, Json(product).into_response())
        },
        Result::Err(e) => {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": 400,
                    "message": e,
                })).into_response()
            )
        },
    }
}

pub async fn insert_product(Json(req): Json<Product>) -> impl IntoResponse {
    match usecases::insert_product(req) {
        Result::Ok(p) =>  (StatusCode::OK, Json(p).into_response()),
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