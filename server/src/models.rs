use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema, ToSchema)]
pub enum Role {
    CUSTOMER,
    VENDOR,
}

impl Default for Role {
    fn default() -> Self {
        Role::CUSTOMER
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub _key: String,
    pub _rev: String,
    pub _id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub role: Role,
}

#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Order {
    pub _key: String,
    pub _rev: String,
    pub _id: String,
    pub date: NaiveDateTime,
    pub user_id: String,
    pub item_id: String,
    pub item_name: String,
    pub quantity: i64,
    pub price: f64,
}

#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub _key: String,
    pub _rev: String,
    pub _id: String,
    pub name: String,
    pub user_id: String,
    pub description: String,
    pub price: f64,
    pub quantity: i64,
}