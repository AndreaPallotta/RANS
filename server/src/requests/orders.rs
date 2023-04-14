use crate::api::{generate_error, ApiResponse};
use crate::db::Database;
use crate::models::{Item, Order};
use arangors::document::options::UpdateOptions;
use arangors::Document;
use axum::{extract::Path, http::StatusCode, Extension, Json};
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Deserialize, Debug, Serialize, ToSchema)]
pub struct AddOrderReq {
    user_id: String,
    item_id: String,
    item_name: String,
    quantity: i64,
    price: f64,
    quantity_diff: i64,
}

#[derive(Deserialize, Debug, Serialize, ToSchema)]
pub struct ItemQuantityUpdate {
    quantity: i64,
}

#[utoipa::path(
    get,
    path = "/api/get_orders/{user_id}",
    params(
        ("user_id" = String, Path, description = "User ID associated with the order")
    ),
    responses(
        (status = 200, description = "Return list of orders based on user id", body = Vec<Order>),
        (status = 404, description = "No orders found", body = ErrorResponse),
        (status = 500, description = "Error in the database query", body = ErrorResponse)
    )
)]
pub async fn get_orders(
    Extension(database): Extension<Database>,
    Path(user_id): Path<String>,
) -> (StatusCode, Json<ApiResponse<Vec<Order>>>) {
    let mut bind_vars: HashMap<&str, Value> = HashMap::new();
    bind_vars.insert("user_id", user_id.to_owned().into());

    match database
        .arango_db
        .aql_bind_vars(
            "FOR order IN Order FILTER order.user_id == @user_id RETURN order",
            bind_vars,
        )
        .await
    {
        Ok(orders) => {
            if orders.is_empty() {
                (StatusCode::NOT_FOUND, generate_error("No orders found"))
            } else {
                (StatusCode::OK, Json(ApiResponse::Success(orders)))
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            generate_error(format!("Error getting item: {}", e.to_string()).as_str()),
        ),
    }
}

#[utoipa::path(
    post,
    path = "/api/add_order",
    request_body = AddOrderReq,
    responses(
        (status = 200, description = "Return created order", body = Order),
        (status = 400, description = "Error creating order. Missing or malformatted attributes", body = ErrorResponse),
        (status = 500, description = "Error querying the database", body = ErrorResponse)
    )
)]
pub async fn add_order(
    Extension(database): Extension<Database>,
    Json(payload): Json<AddOrderReq>,
) -> (StatusCode, Json<ApiResponse<Order>>) {
    let user_id: String = payload.user_id;
    let item_id: String = payload.item_id;
    let item_name: String = payload.item_name;
    let quantity: i64 = payload.quantity;
    let price: f64 = payload.price;
    let diff: i64 = payload.quantity_diff;
    let date: NaiveDateTime = Local::now().naive_local();

    let collection = database.arango_db.collection("Item").await.unwrap();
    let item: Result<Document<Item>, arangors::ClientError> =
        collection.document(&item_id.to_owned()).await;
    match item {
        Ok(item) => {
            if quantity > item.quantity {
                return (
                    StatusCode::BAD_REQUEST,
                    generate_error("Order quantity exceeds item quantity"),
                );
            }
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                generate_error(format!("Error retrieving item: {}", e.to_string()).as_str()),
            );
        }
    }

    let mut bind_vars: HashMap<&str, Value> = HashMap::new();
    bind_vars.insert("user_id", user_id.to_owned().into());
    bind_vars.insert("item_id", item_id.to_owned().into());
    bind_vars.insert("quantity", quantity.to_owned().into());
    bind_vars.insert("price", price.to_owned().into());
    bind_vars.insert("date", to_value(&date).unwrap());
    bind_vars.insert("item_name", item_name.to_owned().into());

    let query: &str = "
    INSERT {
        user_id: @user_id,
        item_id: @item_id,
        item_name: @item_name,
        quantity: @quantity,
        price: @price,
        date: @date
    } INTO Order
    RETURN NEW
    ";

    let result: Result<Vec<Order>, arangors::ClientError> =
        database.arango_db.aql_bind_vars(query, bind_vars).await;

    match result {
        Ok(orders) => {
            if let Some(order) = orders.first() {
                let collection = database.arango_db.collection("Item").await.unwrap();
                let response = collection
                    .update_document(
                        item_id.to_owned().as_str(),
                        ItemQuantityUpdate { quantity: diff },
                        UpdateOptions::builder().return_new(true).build(),
                    )
                    .await;

                match response {
                    Ok(_) => (StatusCode::OK, Json(ApiResponse::Success(order.clone()))),
                    Err(e) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        generate_error(format!("Error creating order: {}", e.to_string()).as_str()),
                    ),
                }
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    generate_error("Error creating order"),
                )
            }
        }
        Err(e) => {
            eprintln!("{:?}", e.to_string());
            (
                StatusCode::BAD_REQUEST,
                generate_error(format!("Error creating order: {}", e.to_string()).as_str()),
            )
        }
    }
}
