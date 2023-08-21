use super::entities::{Result, InsertItemReq, ItemBson, Item};
use bson::{from_document, Bson};
use mongodb::Cursor;
use mongodb::{Client, options::ClientOptions, Database};
use mongodb::bson::{doc, Document};
use bson::oid::ObjectId;

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

pub async fn find_items() -> Vec<Item> {
    let db_conn = dbconnect().await;

    let db: Database;
    match db_conn {
        Ok(r) => db = r,
        Err(e) => panic!("Error: connect to db failed {:?}", e),
    };

    let col = db.collection::<Document>("items");

    let cursor_result = col.find(doc! {}, None).await;
    let mut cursor: Cursor<Document>;
    match cursor_result {
        Ok(r) => cursor = r,
        Err(e) => {
            println!("Error: {:?}", e);
            return Vec::new()
        }
    }

    let mut items: Vec<Item> = Vec::new(); // Initialize an empty vector for items
    while let Ok(next) = cursor.advance().await {
        if !next {
            break
        }

        let item_doc = match cursor.deserialize_current().ok() {
            Some(doc) => doc,
            None => break
        };
        let item: ItemBson = match from_document(item_doc).map_err(|e| format!("Error: deserializing document: {:?}", e)) {
            Ok(i) => i,
            Err(e) => {
                println!("Error: {:?}", e);
                return Vec::new()
            }
        };

        items.push(Item {
            _id: item._id.to_hex(),
            name: item.name,
            description: item.description,
            damage: item.damage,
            level_required: item.level_required,
            price: item.price,
        });
    }

    items
}

pub async fn insert_one_item(req: InsertItemReq) -> Result<ObjectId, String> {
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

    let inserted_id_bson: Bson = match result {
        Ok(r) => r.inserted_id,
        Err(e) => return Result::Err(format!("Error: {:?}", e))
    };

    match inserted_id_bson.as_object_id() {
        Some(id) => Result::Ok(id),
        None => Result::Err(format!("Error: insert one item failed"))
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
        _id: item._id.to_hex(),
        name: item.name,
        description: item.description,
        damage: item.damage,
        level_required: item.level_required,
        price: item.price,
    })
}