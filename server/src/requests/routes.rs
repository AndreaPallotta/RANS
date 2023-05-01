use std::time::Duration;
use crate::logs::set_log;
use crate::requests::{ auth, items, jwt, orders };
use crate::{ db::Database, toml_env::{ Environment, ServerConfig } };
use axum::http::header;
use axum::{
    body::{ Body, Bytes },
    http::{ HeaderMap, HeaderName, Request, Method },
    middleware,
    response::Response,
    routing::{ delete, get, post, put },
    Extension,
    Router,
};
use log::{ debug, error, info, LevelFilter };
use tower_http::{
    classify::ServerErrorsFailureClass,
    compression::CompressionLayer,
    cors::CorsLayer,
    propagate_header::PropagateHeaderLayer,
    trace::TraceLayer,
    validate_request::ValidateRequestHeaderLayer,
};
use tracing::Span;

pub async fn create_routes(database: Database, path: &str, server: &ServerConfig) -> Router {
    set_log(path, LevelFilter::Info);

    let cors = if server.allow_origins().is_none() || server.env == Environment::DEV {
        CorsLayer::permissive()
    } else {
        CorsLayer::new()
            .allow_origin(server.allow_origins().unwrap())
            .allow_headers(vec![header::AUTHORIZATION])
    };

    Router::new()
        .route("/api/auth/login", post(auth::handle_login))
        .route("/api/auth/signup", post(auth::handle_signup))
        .route("/api/auth/refresh/:email", get(jwt::refresh))
        .route(
            "/api/get_item/:name",
            get(items::get_item).route_layer(middleware::from_fn(jwt::jwt_middleware))
        )
        .route(
            "/api/get_items",
            get(items::get_items).route_layer(middleware::from_fn(jwt::jwt_middleware))
        )
        .route(
            "/api/add_item",
            post(items::add_item).route_layer(middleware::from_fn(jwt::jwt_middleware))
        )
        .route(
            "/api/edit_item",
            put(items::edit_item).route_layer(middleware::from_fn(jwt::jwt_middleware))
        )
        .route(
            "/api/delete_item",
            delete(items::delete_item).route_layer(middleware::from_fn(jwt::jwt_middleware))
        )
        .route(
            "/api/get_orders/:user_id",
            get(orders::get_orders).route_layer(middleware::from_fn(jwt::jwt_middleware))
        )
        .route(
            "/api/add_order",
            post(orders::add_order).route_layer(middleware::from_fn(jwt::jwt_middleware))
        )
        .route(
            "/api/delete_orders",
            delete(orders::delete_orders).route_layer(middleware::from_fn(jwt::jwt_middleware))
        )
        .layer(Extension(database))
        .layer(Extension(server.secret.clone()))
        .layer(CompressionLayer::new())
        .layer(PropagateHeaderLayer::new(HeaderName::from_static("x-request-id")))
        .layer(ValidateRequestHeaderLayer::accept("application/json"))
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_request: &Request<Body>| tracing::debug_span!("http-request"))
                .on_request(|request: &Request<Body>, span: &Span| {
                    let method = request.method();

                    let uri = request.uri().path();

                    let user_id = if let Some(user_id_header) = request.headers().get("user_id") {
                        user_id_header.to_str().unwrap_or("Unknown_ID")
                    } else {
                        "Unknown_ID"
                    };

                    if method == Method::OPTIONS {
                        info!("REQUEST - {} | {}", method, uri);
                    } else {
                        info!("REQUEST - {} | {} | {}", user_id, method, uri);
                    }

                    println!("Request: {} {}", method, uri);
                    debug!("{:?} | {:?}", request, span);
                })
                .on_response(|response: &Response, latency: Duration, span: &Span| {
                    let status = response.status().as_u16();
                    println!("{} Response generated in {}ms", status, latency.as_millis());
                    info!("RESPONSE - {} | {}ms", status, latency.as_millis());
                    debug!("{:?} | {:?} | {:?}", response, latency, span);
                })
                .on_body_chunk(|chunk: &Bytes, latency: Duration, span: &Span| {
                    debug!(
                        "CHUNK - sending {} took {}ms: {:?}",
                        chunk.len(),
                        latency.as_millis(),
                        span
                    )
                })
                .on_eos(|trailers: Option<&HeaderMap>, stream_duration: Duration, span: &Span| {
                    debug!(
                        "Stream closed after {:?}ms, {:?}, {:?}",
                        stream_duration.as_millis(),
                        trailers.unwrap(),
                        span
                    );
                })
                .on_failure(|error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                    eprintln!(
                        "Request failed with error {:?} after {}ms",
                        error,
                        latency.as_millis()
                    );
                    error!("FAILURE - {:?} | {}ms", error, latency.as_millis());
                })
        )
}