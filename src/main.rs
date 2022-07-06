mod routes;

use axum::{
    body::Bytes,
    error_handling::HandleErrorLayer,
    http::{header, HeaderValue, StatusCode, Method},
    response::IntoResponse,
    routing::{ get, post },
    BoxError, Router,
};
use std::{
    time::Duration,
};
use tower::ServiceBuilder;
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit, ServiceBuilderExt,
    cors::{CorsLayer, Any},
};

#[tokio::main]
async fn main() {
    println!("Listening...");

    axum::Server::bind(&"127.0.0.1:3001".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap(); 
}

fn app() -> Router {
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_origin(Any);


    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http()
                .on_body_chunk(|chunk: &Bytes, latency: Duration, _: &tracing::Span| {
                    tracing::trace!(size_bytes = chunk.len(), latency = ?latency, "sending body chunk")
                })
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(DefaultOnResponse::new().include_headers(true).latency_unit(LatencyUnit::Micros)),)
        .layer(HandleErrorLayer::new(handle_errors))
        .timeout(Duration::from_secs(10))
        .compression()
        .insert_response_header_if_not_present(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),)
        .layer(cors);

    Router::new()
        .route("/books", get(routes::retrieve))
        .route("/books", post(routes::add))
        .route("/borrowing", post(routes::borrowing_post))
        .route("/borrowing", get(routes::borrowing_get))
        .route("/returning", post(routes::returning))
        .layer(middleware.into_inner())
}

async fn handle_errors(err: BoxError) -> impl IntoResponse {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    }
}

