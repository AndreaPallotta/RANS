use std::time::Duration;

use axum::{
    routing::{get, post, put, delete},
    Router, Extension,
    http::{HeaderName, Request, HeaderMap},
    response::Response, body::{Body, Bytes}
};
use log::{LevelFilter, info, error, debug};
use tracing::Span;
use crate::db::Database;
use crate::logs::set_log;
use crate::requests::{auth, items};
use tower_http::{
    compression::CompressionLayer,
    propagate_header::PropagateHeaderLayer,
    trace::TraceLayer,
    validate_request::ValidateRequestHeaderLayer,
    cors::CorsLayer,
    classify::ServerErrorsFailureClass,
};

pub async fn create_routes(database: Database, path: &str) -> Router {
    set_log(path, LevelFilter::Info);

    Router::new()
        .route("/api/auth/login", post(auth::handle_login))
        .route("/api/auth/signup", post(auth::handle_signup))
        .route("/api/get_item/:name", get(items::get_item))
        .route("/api/get_items", get(items::get_items))
        .route("/api/add_item", post(items::add_item))
        .route("/api/edit_item", put(items::edit_item))
        .route("/api/delete_item", delete(items::delete_item))
        .layer(Extension(database))
        .layer(CompressionLayer::new())
        .layer(PropagateHeaderLayer::new(HeaderName::from_static("x-request-id")))
        .layer(ValidateRequestHeaderLayer::accept("application/json"))
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_request: &Request<Body>| {
                    tracing::debug_span!("http-request")
                })
                .on_request(|request: &Request<Body>, span: &Span| {
                    let method = request.method().to_string();
                    let uri = request.uri().path();
                    let body = request.body();
                    println!("Request: {} {}", method, uri);
                    info!("{}, {}, {:?}", method, uri, body);
                    debug!("{:?}, {:?}", request, span);
                })
                .on_response(|response: &Response, latency: Duration, span: &Span| {
                    let status = response.status().as_u16();
                    let body = response.body();
                    println!("{} Response generated in {}ms", status, latency.as_millis());
                    info!("{}, {:?}, {}ms", status, body, latency.as_millis());
                    debug!("{:?}, {:?}, {:?}", response, latency, span);
                })
                .on_body_chunk(|chunk: &Bytes, latency: Duration, span: &Span| {
                    debug!("sending {} took {}ms: {:?}", chunk.len(), latency.as_millis(), span)
                })
                .on_eos(|trailers: Option<&HeaderMap>, stream_duration: Duration, span: &Span| {
                    debug!("Stream closed after {:?}ms, {:?}, {:?}", stream_duration.as_millis(), trailers.unwrap(), span);
                })
                .on_failure(
                    |error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                        eprintln!("Request failed with error {:?} after {}ms", error, latency.as_millis());
                        error!(
                            "{:?}, {}ms",
                            error,
                            latency.as_millis()
                        );
                }),
        )
}