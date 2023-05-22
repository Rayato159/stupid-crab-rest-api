use std::net::SocketAddr;
use axum::{
    routing::get,
    Router,
};

pub mod products;

#[tokio::main]
pub async fn start() {
    let app = Router::new().route("/:product_id", get(products::handlers::find_one_product));
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("server is running on -> {:?}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}