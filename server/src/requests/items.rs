use crate::api::{generate_error, ApiResponse};
use crate::db::Database;
use crate::models::Item;
use arangors::document::options::{RemoveOptions, UpdateOptions};
use axum::extract::Path;
use axum::Extension;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Number, Value};
use std::collections::HashMap;
use urlencoding::decode;
use utoipa::ToSchema;

#[derive(Deserialize, Debug, Serialize, ToSchema)]
pub struct GetItemReq {
    id: String,
}

#[derive(Deserialize, Debug, Serialize, ToSchema)]
pub struct AddItemReq {
    name: String,
    description: String,
    price: f64,
    quantity: i64,
}

#[derive(Deserialize, Debug, Serialize, Clone, ToSchema)]
pub struct UpdateItemReq {
    id: String,
    name: Option<String>,
    description: Option<String>,
    price: Option<f64>,
    quantity: Option<i64>,
}

#[derive(Deserialize, Debug, Serialize, ToSchema)]
pub struct DeleteItemReq {
    id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ItemUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<i64>,
}

#[utoipa::path(
    get,
    path = "/api/get_item/{name}",
    params(
        ("name" = String, Path, description = "Item Name")
    ),
    responses(
        (status = 200, description = "Return list of items that loosely match the name", body = Vec<Item>),
        (status = 404, description = "No results found", body = ErrorResponse),
        (status = 500, description = "Error in the database query", body = ErrorResponse)
    )
)]
pub async fn get_item(
    Extension(database): Extension<Database>,
    Path(name): Path<String>,
) -> (StatusCode, Json<ApiResponse<Vec<Item>>>) {
    let decoded_name = decode(name.as_str()).expect("UTF-8");

    let mut bind_vars: HashMap<&str, Value> = HashMap::new();
    bind_vars.insert("name", Value::String(decoded_name.into_owned()));

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

#[utoipa::path(
    get,
    path = "/api/get_items",
    responses(
        (status = 200, description = "Return all items in the database", body = Vec<Item>),
        (status = 500, description = "Error in the database query", body = ErrorResponse)
    )
)]
pub async fn get_items(
    Extension(database): Extension<Database>,
) -> (StatusCode, Json<ApiResponse<Vec<Item>>>) {
    match database
        .arango_db
        .aql_str("FOR item IN Item RETURN item")
        .await
    {
        Ok(items) => (StatusCode::OK, Json(ApiResponse::Success(items))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            generate_error(format!("Error getting items: {}", e.to_string()).as_str()),
        ),
    }
}

#[utoipa::path(
    post,
    path = "/api/add_item",
    request_body = AddItemReq,
    responses(
        (status = 200, description = "Return created item", body = Item),
        (status = 500, description = "Error parsing request body. Missing or malformatted attributes", body = ErrorResponse),
        (status = 500, description = "Error in the database query", body = ErrorResponse)
    )
)]
pub async fn add_item(
    Extension(database): Extension<Database>,
    Json(payload): Json<AddItemReq>,
) -> (StatusCode, Json<ApiResponse<Item>>) {
    let name: String = payload.name;
    let description: String = payload.description;
    let price: f64 = payload.price;
    let quantity: i64 = payload.quantity;

    let mut bind_vars: HashMap<&str, Value> = HashMap::new();
    bind_vars.insert("name", Value::String(name.clone()));
    bind_vars.insert("description", Value::String(description));
    bind_vars.insert("price", Value::Number(Number::from_f64(price).unwrap()));
    bind_vars.insert("quantity", Value::Number(Number::from(quantity)));

    let query = "
    INSERT {
        name: @name,
        description: @description,
        price: @price,
        quantity: @quantity
    } INTO Item
    RETURN NEW
    ";

    let result: Result<Vec<Item>, arangors::ClientError> =
        database.arango_db.aql_bind_vars(query, bind_vars).await;

    match result {
        Ok(items) => {
            if let Some(item) = items.first() {
                (StatusCode::OK, Json(ApiResponse::Success(item.clone())))
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    generate_error("Error creating item"),
                )
            }
        }
        Err(e) => {
            eprintln!("{:?}", e.to_string());
            (
                StatusCode::BAD_REQUEST,
                generate_error(format!("Error creating item: Name {} already used", name).as_str()),
            )
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/edit_item",
    request_body = UpdateItemReq,
    responses(
        (status = 200, description = "Return created item", body = Item),
        (status = 404, description = "Error editing item. Item does not exist in database", body = ErrorResponse),
        (status = 500, description = "Error in the database query", body = ErrorResponse)
    )
)]
pub async fn edit_item(
    Extension(database): Extension<Database>,
    Json(payload): Json<UpdateItemReq>,
) -> (StatusCode, Json<ApiResponse<Value>>) {
    let id = payload.id;
    let name = payload.name;
    let description = payload.description;
    let price = payload.price;
    let quantity = payload.quantity;

    let params = ItemUpdate {
        name,
        description,
        price,
        quantity,
    };

    let patch = json!(&params);

    let collection = database.arango_db.collection("Item").await.unwrap();
    let response = collection
        .update_document(
            id.to_owned().as_str(),
            patch,
            UpdateOptions::builder().return_new(true).build(),
        )
        .await;

    match response {
        Ok(response) => {
            if let Some(item) = response.new_doc() {
                (StatusCode::OK, Json(ApiResponse::Success(item.clone())))
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    generate_error("Error updating item"),
                )
            }
        }
        Err(e) => {
            eprintln!("Error updating item: {}", e.to_string());
            (
                StatusCode::NOT_FOUND,
                generate_error(format!("Error updating item: id {} not found", id).as_str()),
            )
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/delete_item",
    request_body = DeleteItemReq,
    responses(
        (status = 200, description = "Return deleted item name", body = String),
        (status = 404, description = "Error deleting item. Item does not exist in database", body = ErrorResponse),
        (status = 500, description = "Error in the database query", body = ErrorResponse)
    )
)]
pub async fn delete_item(
    Extension(database): Extension<Database>,
    Json(payload): Json<DeleteItemReq>,
) -> (StatusCode, Json<ApiResponse<Value>>) {
    let id = payload.id;
    let collection = database.arango_db.collection("Item").await.unwrap();

    match collection
        .remove_document(
            id.to_owned().as_str(),
            RemoveOptions::builder().return_old(true).build(),
            None,
        )
        .await
    {
        Ok(res) => {
            if let Some(old_doc) = res.old_doc() {
                let item: &Item = old_doc;
                (
                    StatusCode::OK,
                    Json(ApiResponse::Success(json!({ "name": item.name }))),
                )
            } else {
                (
                    StatusCode::NOT_FOUND,
                    generate_error("Item to delete not found"),
                )
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            generate_error(format!("Error deleting item: {}", e.to_string()).as_str()),
        ),
    }
}
