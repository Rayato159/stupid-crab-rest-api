use super::entities;

pub fn find_one_product(product_id: i32) -> entities::Result<entities::Product, String> {
    let mut products: Vec<entities::Product> = Vec::new();

    products.push(entities::Product {
        id: 1,
        title: String::from("Tom Yum Kung"),
        description: String::from("Thai's food"),
    });
    products.push(entities::Product {
        id: 2,
        title: String::from("Shushi"),
        description: String::from("Japanese's food"),
    });
    products.push(entities::Product {
        id: 3,
        title: String::from("Roti"),
        description: String::from("Indian's food"),
    });

    for product in products {
        if product.id == product_id {
            return entities::Result::Ok(product)
        };
    }
    entities::Result::Err(format!("product_id: {} not found", product_id))
}