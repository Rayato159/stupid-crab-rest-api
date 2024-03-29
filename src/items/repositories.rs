use super::entities::{Result, InsertItemReq, ItemBson, Item};
use bson::{from_document, Bson};
use mongodb::Cursor;
use mongodb::results::{DeleteResult, UpdateResult};
use mongodb::bson::{doc, Document};
use bson::oid::ObjectId;
use tracing::info;
use crate::config::database::dbconnect;

pub async fn find_items() -> Vec<Item> {
    let db = match dbconnect().await {
        Ok(r) => r,
        Err(e) => panic!("Error: connect to db failed {:?}", e),
    };

    let col = db.collection::<Document>("items");

    let cursor_result = col.find(doc! {}, None).await;
    let mut cursor: Cursor<Document>;
    match cursor_result {
        Ok(r) => cursor = r,
        Err(e) => {
            info!("Error: {:?}", e);
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
                info!("Error: {:?}", e);
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
    let db = match dbconnect().await {
        Ok(r) => r,
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
    let db = match dbconnect().await {
        Ok(r) => r,
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

pub async fn update_one_item(req: ItemBson) -> Result<UpdateResult, String> {
    let db = match dbconnect().await {
        Ok(r) => r,
        Err(e) => panic!("Error: connect to db failed {:?}", e),
    };

    let col = db.collection::<Document>("items");

    let mut updated_fields = doc! {};

    if req.name != "" {
        updated_fields.insert("name", req.name);
    }
    if req.description != "" {
        updated_fields.insert("description", req.description);
    }
    if req.damage > 0 {
        updated_fields.insert("damage", req.damage);
    }
    if req.level_required > 0 {
        updated_fields.insert("level_required", req.level_required);
    }
    if req.price > 0 {
        updated_fields.insert("price", req.price);
    }

    match col.update_one(doc! {"_id": req._id}, doc! {"$set": updated_fields}, None).await {
        Ok(r) => Result::Ok(r),
        Err(e) => {
            info!("Error: update one item failed: {:?}", e);
            Result::Err(format!("Error: update one item failed"))
        }
    }
}

pub async fn delete_one_item(item_id: ObjectId) -> Result<DeleteResult, String> {
    let db = match dbconnect().await {
        Ok(r) => r,
        Err(e) => panic!("Error: connect to db failed {:?}", e),
    };

    let col = db.collection::<Document>("items");

    match col.delete_one(doc! {"_id": item_id}, None).await {
        Ok(r) => Result::Ok(r),
        Err(e) => {
            info!("Error: delete item failed: {:?}", e);
            Result::Err(format!("Error: delete item failed"))
        }
    }
}