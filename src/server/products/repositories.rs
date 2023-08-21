use super::entities::{Product, Result, InsertProductReq};
use mongodb::{Client, options::ClientOptions, Database};
use mongodb::bson::{doc, Document};

pub async fn dbconnect() -> mongodb::error::Result<Database> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://root:123456@127.0.0.1:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    let db = client.database("stupid_crab_db");

    Ok(db)
}

pub async fn insert_one_product(mut req: InsertProductReq) -> Result<String, String> {
    if req.title == "" {
        return Result::Err(format!("insert product {:?} failed", req))
    }

    let dbConn = dbconnect().await;

    let mut db: Database;
    match dbConn {
        Ok(r) => db = r,
        Err(e) => panic!("Error: connect to db failed {:?}", e),
    };

    let col = db.collection::<Document>("products");

    let result = col.insert_one(doc!{
        "title": req.title,
        "description": req.description,
    }, None).await;

    match result {
        Ok(r) => Result::Ok(format!("Insert one: {:?}", r)),
        Err(e) => Result::Err(format!("Error: {:?}", e))
    }
}

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