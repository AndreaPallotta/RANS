use crate::{
    api::{generate_error, ApiResponse},
    constants::PROD_CONFIG_PATH,
    db::Database,
    models::User,
    toml_env::Config,
};
use axum::{
    extract::Path,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Extension, Json,
};
use jsonwebtoken::{
    decode, encode,
    errors::{Error, ErrorKind},
    DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

use super::auth::AuthRes;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
}

pub fn generate_jwt(sub: &String, secret: &String) -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header::default();
    let claims = Claims {
        sub: sub.to_string(),
        iat: chrono::Utc::now().timestamp() as usize,
        exp: (chrono::Utc::now() + chrono::Duration::minutes(15)).timestamp() as usize,
    };
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn validate_jwt(token: &String, secret: &String) -> Result<bool, Error> {
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    let result = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    );

    match result {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.kind() == &ErrorKind::ExpiredSignature {
                Err(ErrorKind::ExpiredSignature.into())
            } else {
                Err(ErrorKind::InvalidToken.into())
            }
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/auth/refresh",
    params(
        ("email" = String, Path, description = "user email")
    ),
    responses(
        (status = 200, description = "Return authenticated user", body = AuthRes),
        (status = 500, description = "Error generating jwt", body = ErrorResponse)
    )
)]
pub async fn refresh(
    Extension(database): Extension<Database>,
    Extension(secret): Extension<String>,
    Path(email): Path<String>,
) -> (StatusCode, Json<ApiResponse<AuthRes>>) {
    let users: Vec<User> = database
        .arango_db
        .aql_bind_vars(
            "FOR user IN User FILTER user.email == @email RETURN user",
            HashMap::from([("email", email.to_owned().into())]),
        )
        .await
        .unwrap();

    let token = generate_jwt(&email, &secret);

    let response = token
        .map(|jwt| {
            users.first().map_or_else(
                || {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        generate_error("Error generating token: no user found"),
                    )
                },
                |user| {
                    (
                        StatusCode::OK,
                        Json(ApiResponse::Success(AuthRes::new(user.to_owned(), jwt))),
                    )
                },
            )
        })
        .unwrap_or_else(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                generate_error(format!("Error generating token: {}", e.to_string()).as_str()),
            )
        });

    response
}

pub async fn validate_jwt_route(
    Extension(secret): Extension<String>,
    Path(token): Path<String>,
) -> (StatusCode, Json<ApiResponse<bool>>) {
    match validate_jwt(&token, &secret) {
        Ok(_) => (StatusCode::OK, Json(ApiResponse::Success(true))),
        Err(e) => {
            eprintln!("Error validating JWT token: {:?}", e.to_string());
            (
                StatusCode::UNAUTHORIZED,
                generate_error("Invalid JWT Token"),
            )
        }
    }
}

pub async fn jwt_middleware<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let parsed_config = match Config::parse(PROD_CONFIG_PATH) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error parsing configuration: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if parsed_config.server.env.is_dev() {
        return Ok(next.run(req).await);
    }

    let secret = parsed_config.server.secret;

    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token = if let Some(auth_header) = auth_header {
        auth_header.split_whitespace().nth(1)
    } else {
        eprintln!("Error validating token. Bearer token missing in request");
        return Err(StatusCode::UNAUTHORIZED);
    };

    match token {
        Some(tok) => match validate_jwt(&tok.to_string(), &secret) {
            Ok(_) => return Ok(next.run(req).await),
            Err(e) => {
                eprintln!("Error validating JWT token: {:?}", e.to_string());
                return Err(StatusCode::UNAUTHORIZED);
            }
        },
        None => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
}
