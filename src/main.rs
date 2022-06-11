use club_library::retrieve_books;

#[tokio::main]
async fn main() {
    println!("Application started!");
    retrieve_books().await.unwrap();
}