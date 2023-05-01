use axum::Json;
use schemars::JsonSchema;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, JsonSchema, ToSchema)]
pub struct ErrorResponse {
    pub error_msg: String,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case", tag = "result", content = "content")]
pub enum ApiResponse<T> {
    Success(T),
    Error(ErrorResponse),
}

pub fn generate_error<T>(msg: &str) -> Json<ApiResponse<T>> {
    Json(
        ApiResponse::Error(ErrorResponse {
            error_msg: msg.to_string(),
        })
    )
}