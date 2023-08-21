use axum::extract::Path;
use axum::response::{Json, IntoResponse};

use super::entities::InsertItemReq;
use super::usecases;

// pub async fn find_products() -> impl IntoResponse {
//     (StatusCode::OK, Json(usecases::find_products()).into_response())
// }

pub async fn find_one_item(Path(item_id): Path<String>) -> impl IntoResponse {
    usecases::find_one_item(item_id).await
}

pub async fn insert_one_item(Json(req): Json<InsertItemReq>) -> impl IntoResponse {
    usecases::insert_one_item(req).await
}

// pub async fn update_product(Json(req): Json<Item>) -> impl IntoResponse {
//     match usecases::update_product(req) {
//         Result::Ok(p) =>  (StatusCode::OK, Json(p).into_response()),
//         Result::Err(e) => {
//             (
//                 StatusCode::BAD_REQUEST,
//                 Json(json!({
//                     "status": 400,
//                     "message": e,
//                 })).into_response()
//             )
//         }
//     }
// }

// pub async fn delete_product(Path(product_id): Path<i32>) -> impl IntoResponse {
//     match usecases::delete_product(product_id) {
//         Result::Ok(p) =>  (StatusCode::OK, Json(p).into_response()),
//         Result::Err(e) => {
//             (
//                 StatusCode::BAD_REQUEST,
//                 Json(json!({
//                     "status": 400,
//                     "message": e,
//                 })).into_response()
//             )
//         }
//     }
// }