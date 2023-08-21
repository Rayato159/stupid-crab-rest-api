use super::entities::{Product, Result, InsertProductReq};
use super::repositories::{products_db, dbconnect, insert_one_product};
use mongodb::bson::{doc, Document};
use mongodb::Database;

pub fn find_products() -> Vec<Product> {
    products_db()
}

pub fn find_one_product(product_id: i32) -> Result<Product, String> {
    let mut products = products_db();

    for product in products {
        if product.id == product_id {
            return Result::Ok(product)
        };
    }
    Result::Err(format!("product_id: {} not found", product_id))
}

pub async fn insert_product(mut req: InsertProductReq) -> Result<String, String> {
    insert_one_product(req).await
}

pub fn update_product(req: Product) -> Result<Product, String> {
    let products = products_db();

    for mut product in products {
        if req.id == product.id {
            if req.title != String::from("") {
                product.title = req.title
            }
            if req.description != String::from("") {
                product.description = req.description
            }
            return Result::Ok(product)
        }
    }
    Result::Err(format!("product_id {} not found", req.id))
}

pub fn delete_product(product_id: i32) -> Result<Vec<Product>, String> {
    let mut products = products_db();

    for (i, product) in products.iter().enumerate() {
        if product_id == product.id {
            products.remove(i);
            return Result::Ok(products)
        }
    }
    Result::Err(format!("product_id {} not found", product_id))
}