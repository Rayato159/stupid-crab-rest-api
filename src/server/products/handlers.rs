use axum::{
    response::{Json, IntoResponse},
    extract::{Path},
    http::{StatusCode},
};
use serde_json::json;

use super::entities;
use super::usecases;

pub async fn find_one_product(Path(product_id): Path<i32>) -> impl IntoResponse {
    let product_id_int = product_id.abs();
    match usecases::find_one_product(product_id_int) {
        entities::Result::Ok(product) => {
            (StatusCode::OK, Json(product).into_response())
        },
        entities::Result::Err(e) => {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": 200,
                    "message": e,
                })).into_response()
            )
        },
    }
}