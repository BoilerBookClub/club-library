mod routes;

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/books", get(routes::retrieve))
        .route("/books", post(routes::add))
        .route("/borrowing", post(routes::borrowing_post))
        .route("/borrowing", get(routes::borrowing_get))
        .route("/returning", post(routes::returning));

    println!("Listening...");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap(); 
}