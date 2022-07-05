use axum::response::{IntoResponse, Json};
use axum::extract::Query;
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AddRequest {
    title: String,
    author: String,
    genre: String,
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
pub struct BorrowReturnRequest {
    id: String,
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
pub struct IdentifiedRequest {
    name: String,
    email: String,
}

pub async fn retrieve() -> Json<Vec<club_library::Book>> {
    Json(club_library::retrieve_books().await.unwrap())
}

pub async fn add(Query(req): Query<AddRequest>) -> impl IntoResponse {
    match club_library::add_book(req.title, req.author, 
                                 req.genre, req.name, 
                                 req.email).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

pub async fn borrowing_post(Query(req): Query<BorrowReturnRequest>) -> impl IntoResponse {
    match club_library::borrow_book(req.id, req.name.as_str(), 
                                    req.email.as_str()).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

pub async fn borrowing_get(Query(req): Query<IdentifiedRequest>) -> Json<Vec<club_library::Book>> {
    Json(club_library::retrieve_borrowing_books(req.name.as_str(), req.email.as_str()).await.unwrap())
}

pub async fn returning(Query(req): Query<BorrowReturnRequest>) -> impl IntoResponse {
    match club_library::return_book(req.id, req.name.as_str(), 
                                    req.email.as_str()).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
