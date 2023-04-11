use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub _key: String,
    pub _rev :String,
    pub _id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Order {
    pub _key: String,
    pub _rev :String,
    pub _id: String,
    pub date: NaiveDateTime,
    pub user_id: String,
    pub item_ids: Vec<String>,
    pub quantity: i64,
}

#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub _key: String,
    pub _rev :String,
    pub _id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: i64,
}
