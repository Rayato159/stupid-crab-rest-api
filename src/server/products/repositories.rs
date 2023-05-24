use super::entities::{Product};

pub fn products_db() -> Vec<Product> {
    vec![
        Product {
            id: 1,
            title: String::from("Tom Yum Kung"),
            description: String::from("Thai's food"),
        },
        Product {
            id: 2,
            title: String::from("Shushi"),
            description: String::from("Japanese's food"),
        },
        Product {
            id: 3,
            title: String::from("Roti"),
            description: String::from("Indian's food"),
        }
    ]
}