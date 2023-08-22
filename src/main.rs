use std::net::SocketAddr;
use axum::{
    http::{Method, StatusCode},
    routing::{get, post, patch, delete},
    response::{Json, IntoResponse},
    Router,
};
use serde_json::json;
use tower_http::cors::{CorsLayer, Origin};

pub mod items;
pub mod config;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/", get(|| async move {
            (
                StatusCode::OK,
                Json(json!({
                    "app": "stupid-crab-rest-api",
                    "version": "v0.1.0",
                })).into_response()
            )
        })).layer(
            CorsLayer::new()
                .allow_origin(Origin::exact(
                    "*".parse().unwrap(),
                ))
                .allow_methods(
                    [
                        Method::GET,
                        Method::POST,
                        Method::PUT,
                        Method::PATCH,
                        Method::DELETE
                    ]
                ),
        )
        .route("/item", post(items::handlers::insert_one_item))
        .route("/:item_id/item", get(items::handlers::find_one_item))
        .route("/:item_id/item", patch(items::handlers::update_one_item))
        .route("/item", get(items::handlers::find_items))
        .route("/:item_id/item", delete(items::handlers::delete_one_item));
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("server is running on -> {:?}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}