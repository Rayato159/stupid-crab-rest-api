use super::entities::{Product, Result};

pub fn find_one_product(product_id: i32) -> Result<Product, String> {
    let mut products: Vec<Product> = Vec::new();

    products.push(Product {
        id: 1,
        title: String::from("Tom Yum Kung"),
        description: String::from("Thai's food"),
    });
    products.push(Product {
        id: 2,
        title: String::from("Shushi"),
        description: String::from("Japanese's food"),
    });
    products.push(Product {
        id: 3,
        title: String::from("Roti"),
        description: String::from("Indian's food"),
    });

    for product in products {
        if product.id == product_id {
            return Result::Ok(product)
        };
    }
    Result::Err(format!("product_id: {} not found", product_id))
}

pub fn insert_product(mut req: Product) -> Result<Product, String> {
    if req.title == "" {
        return Result::Err(format!("isnert product {:?} failed", req))
    }
    req.id = 1;
    Result::Ok(req)
}