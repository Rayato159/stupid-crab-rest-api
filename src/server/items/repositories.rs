use super::entities::{Result, InsertItemReq, ItemBson, Item};
use bson::from_document;
use mongodb::{Client, options::ClientOptions, Database};
use mongodb::bson::{doc, Document};
use bson::oid::ObjectId;
use serde::Deserialize;

pub async fn dbconnect() -> mongodb::error::Result<Database> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://root:123456@127.0.0.1:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    let db = client.database("crab_db");

    Ok(db)
}

pub async fn insert_one_item(req: InsertItemReq) -> Result<String, String> {
    let db_conn = dbconnect().await;

    let db: Database;
    match db_conn {
        Ok(r) => db = r,
        Err(e) => panic!("Error: connect to db failed {:?}", e),
    };

    let col = db.collection::<Document>("items");

    let result = col.insert_one(doc!{
        "name": req.name,
        "description": req.description,
        "damage": req.damage,
        "level_required": req.level_required,
        "price": req.price
    }, None).await;

    match result {
        Ok(r) => Result::Ok(format!("Insert one: {:?}", r)),
        Err(e) => Result::Err(format!("Error: {:?}", e))
    }
}

pub async fn find_one_item(item_id: ObjectId) -> Result<Item, String> {
    let db_conn = dbconnect().await;

    let db: Database;
    match db_conn {
        Ok(r) => db = r,
        Err(e) => panic!("Error: connect to db failed {:?}", e),
    };

    let col = db.collection::<Document>("items");

    let cursor = col.find_one(doc! {"_id": item_id}, None).await;
    let doc: Option<Document> = match cursor {
        Ok(r) => r,
        Err(e) => return Result::Err(format!("Error: {:?}", e))
    };

    let item: ItemBson = match doc {
        Some(r) => match from_document(r).map_err(|e| format!("Error: deserializing document: {:?}", e)) {
            Ok(i) => i,
            Err(e) => return Result::Err(format!("Error: {:?}", e))
        },
        None => return Result::Err(String::from("Error: item not found")),
    };

    Result::Ok(Item {
        _id: item._id.to_string(),
        name: item.name,
        description: item.description,
        damage: item.damage,
        level_required: item.level_required,
        price: item.price,
    })
}