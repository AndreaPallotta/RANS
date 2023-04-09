use std::collections::HashMap;
use axum::Extension;
use axum::extract::Path;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::{Value, Number, json};
use crate::db::Database;
use crate::models::Item;
use crate::api::{ApiResponse, generate_error};
use arangors::document::{
    options::{RemoveOptions, UpdateOptions}
};

#[derive(Deserialize, Debug, Serialize)]
pub struct GetItemReq {
    id: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct AddItemReq {
    name: String,
    description: String,
    price: f64,
    quantity: i64,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct UpdateItemReq {
    id: String,
    name: Option<String>,
    description: Option<String>,
    price: Option<f64>,
    quantity: Option<i64>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DeleteItemReq {
    id: String,
}

#[derive(Debug, Serialize)]
struct ItemUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<i64>,
}

pub async fn get_item(Extension(database): Extension<Database>, Path(name): Path<String>) -> (StatusCode, Json<ApiResponse<Vec<Item>>>) {
    let mut bind_vars: HashMap<&str, Value> = HashMap::new();
    bind_vars.insert("name", name.to_owned().into());

    match database.arango_db.aql_bind_vars("FOR item IN Item FILTER LOWER(item.name) LIKE CONCAT('%', LOWER(@name), '%') RETURN item", bind_vars).await {
        Ok(items) => {
            if items.is_empty() {
                (StatusCode::NOT_FOUND, generate_error("No Item Matches Provided Name"))
            } else {
                (StatusCode::OK, Json(ApiResponse::Success(items)))
            }
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, generate_error(format!("Error getting item: {}", e.to_string()).as_str()))
        }
    }
}

pub async fn get_items(Extension(database): Extension<Database>) -> (StatusCode, Json<ApiResponse<Vec<Item>>>) {
    match database.arango_db.aql_str("FOR item IN Item RETURN item").await {
        Ok(items) => {
            (StatusCode::OK, Json(ApiResponse::Success(items)))
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, generate_error(format!("Error getting items: {}", e.to_string()).as_str()))
        }
    }
}

pub async fn add_item(Extension(database): Extension<Database>, Json(payload): Json<AddItemReq>) -> (StatusCode, Json<ApiResponse<Item>>) {
    let name: String = payload.name;
    let description: String = payload.description;
    let price: f64 = payload.price;
    let quantity: i64 = payload.quantity;

    let mut bind_vars: HashMap<&str, Value> = HashMap::new();
    bind_vars.insert("name", Value::String(name.clone()));
    bind_vars.insert("description", Value::String(description));
    bind_vars.insert("price", Value::Number(Number::from_f64(price).unwrap()));
    bind_vars.insert("quantity", Value::Number(Number::from(quantity)));

    let query =
    "
    INSERT {
        name: @name,
        description: @description,
        price: @price,
        quantity: @quantity
    } INTO Item
    RETURN NEW
    ";

    let result: Result<Vec<Item>, arangors::ClientError> = database.arango_db.aql_bind_vars(query, bind_vars).await;

    match result {
        Ok(items) => {
            if let Some(item) = items.first() {
                (StatusCode::OK, Json(ApiResponse::Success(item.clone())))
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, generate_error("Error creating item"))
            }
        }
        Err(e) => {
            eprintln!("{:?}", e.to_string());
            (StatusCode::BAD_REQUEST, generate_error(format!("Error creating item: Name {} already used", name).as_str()))
        }
    }
}

pub async fn edit_item(Extension(database): Extension<Database>, Json(payload): Json<UpdateItemReq>) -> (StatusCode, Json<ApiResponse<Value>>) {
    let id = payload.id;
    let name = payload.name;
    let description = payload.description;
    let price = payload.price;
    println!("price: {:?}", price);
    let quantity = payload.quantity;
    println!("quantity: {:?}", quantity);

    let params = ItemUpdate {
        name,
        description,
        price,
        quantity,
    };

    let patch = json!(&params);

    let collection = database.arango_db.collection("Item").await.unwrap();
    let response = collection.update_document(id.to_owned().as_str(), patch, UpdateOptions::builder().return_new(true).build()).await;

    match response {
        Ok(response) => {
            if let Some(item) = response.new_doc() {
                (StatusCode::OK, Json(ApiResponse::Success(item.clone())))
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, generate_error("Error updating item"))
            }
        },
        Err(e) => {
            eprintln!("Error updating item: {}", e.to_string());
            (StatusCode::NOT_FOUND, generate_error(format!("Error updating item: id {} not found", id).as_str()))
        }
    }

}

pub async fn delete_item(Extension(database): Extension<Database>, Json(payload): Json<DeleteItemReq>) -> (StatusCode, Json<ApiResponse<Value>>) {
    let id = payload.id;
    let collection = database.arango_db.collection("Item").await.unwrap();

    match collection.remove_document(id.to_owned().as_str(), RemoveOptions::builder().return_old(true).build(), None).await {
        Ok(res) => {
            if let Some(old_doc) = res.old_doc() {
                let item: &Item = old_doc;
                (StatusCode::OK, Json(ApiResponse::Success(json!({ "name": item.name }))))
            } else {
                (StatusCode::NOT_FOUND, generate_error("Item to delete not found"))
            }
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, generate_error(format!("Error deleting item: {}", e.to_string()).as_str()))
        }
    }
}