use axum::{
    response::{Json, IntoResponse},
    extract::{Path},
    http::{StatusCode},
};
use serde_json::json;

use super::entities::{Product, Result, InsertProductReq};
use super::usecases;

pub async fn find_products() -> impl IntoResponse {
    (StatusCode::OK, Json(usecases::find_products()).into_response())
}

pub async fn find_one_product(Path(product_id): Path<i32>) -> impl IntoResponse {
    match usecases::find_one_product(product_id) {
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

pub async fn insert_product(Json(req): Json<InsertProductReq>) -> impl IntoResponse {
    match usecases::insert_product(req).await {
        Result::Ok(msg) =>  (
            StatusCode::OK,
            Json(json!({
                "message": msg,
            })).into_response()
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

pub async fn update_product(Json(req): Json<Product>) -> impl IntoResponse {
    match usecases::update_product(req) {
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

pub async fn delete_product(Path(product_id): Path<i32>) -> impl IntoResponse {
    match usecases::delete_product(product_id) {
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